use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub enum StoreError {
    StoreExists,
    StoreNotFound,
}

use crate::{
    entity::EntityBuilder,
    sparse_set::SparseSet,
    system::SystemRuntime,
    traits::{Component, Plugin},
};

pub struct App {
    next_entity_id: usize,
    components: HashMap<TypeId, Box<dyn Any>>,
    systems: HashMap<SystemRuntime, Vec<fn(&mut Self)>>,
}

impl App {
    pub fn new() -> Self {
        let systems = HashMap::new();

        Self {
            next_entity_id: 0,
            components: HashMap::new(),
            systems,
        }
    }

    pub fn run(mut self) {
        // Handle startup systems
        let mut queue = vec![];

        let startup_systems = self.systems.get(&SystemRuntime::Startup);
        if let Some(startup_systems) = startup_systems {
            for system in startup_systems {
                queue.push(system.clone());
            }
        }

        for f in queue {
            f(&mut self);
        }

        // Mainloop
        loop {
            self.step();
        }
    }

    fn step(&self) {}

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) {
        plugin.attach(self);
    }

    pub fn component_ids(&self) -> Vec<&TypeId> {
        self.components.keys().collect::<Vec<&TypeId>>()
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        EntityBuilder::new(entity_id, self)
    }

    pub fn add_component<C: Component>(&mut self, entity_id: usize, component: C) {
        let type_id = TypeId::of::<C>();

        let store = self
            .components
            .entry(type_id)
            .or_insert_with(|| Box::new(SparseSet::<C>::new()));

        let store = store.downcast_mut::<SparseSet<C>>().unwrap();

        store.insert(entity_id, component);
    }

    pub fn get_component<C: Component>(&self, entity_id: usize) -> Option<&C> {
        let type_id = TypeId::of::<C>();

        let store = self.components.get(&type_id)?;
        let store = store.downcast_ref::<SparseSet<C>>()?;

        store.get(entity_id)
    }

    pub fn get_component_mut<C: Component>(&mut self, entity_id: usize) -> Option<&mut C> {
        let type_id = TypeId::of::<C>();

        let store = self.components.get_mut(&type_id)?;
        let store = store.downcast_mut::<SparseSet<C>>()?;

        store.get_mut(entity_id)
    }

    pub fn add_system(&mut self, runtime: SystemRuntime, system: fn(&mut Self)) -> &mut Self {
        match self.systems.get_mut(&runtime) {
            Some(system_vec) => {
                system_vec.push(system);
            }
            None => {
                self.systems.insert(runtime, vec![system]);
            }
        }
        self
    }
}
