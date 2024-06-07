use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub enum StoreError {
    StoreExists,
    StoreNotFound,
}

use crate::{entity::EntityBuilder, sparse_set::SparseSet, traits::Component};

pub struct App {
    next_entity_id: usize,
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            components: HashMap::new(),
        }
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
}
