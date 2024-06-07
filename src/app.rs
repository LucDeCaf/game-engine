use std::{any::TypeId, collections::HashMap};

pub enum StoreError {
    StoreExists,
}

use crate::{
    entity::EntityBuilder,
    sparse_set::SparseSet,
    traits::{Component, ComponentStore, ToAny},
};

pub struct App {
    next_entity_id: usize,
    component_stores: HashMap<TypeId, Box<dyn ComponentStore>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            component_stores: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        EntityBuilder::new(entity_id, self)
    }

    pub fn add_component<C: Component>(&mut self, entity_id: usize, component: C) {
        let type_id = TypeId::of::<C>();
        match self.component_stores.get_mut(&type_id) {
            Some(store) => {
                let store_any = store.as_any_mut();

                if let Some(store) = store_any.downcast_mut::<SparseSet<C>>() {
                    store.insert(entity_id, component);
                }
            }
            None => {
                let mut store = SparseSet::<C>::new();
                store.insert(entity_id, component);
                self.component_stores.insert(type_id, Box::new(store));
            }
        }
    }

    pub fn get_component<C: Component>(&self, entity_id: usize) -> Option<&C> {
        let type_id = TypeId::of::<C>();
        match self.component_stores.get(&type_id) {
            Some(store) => {
                let store_any = store.as_any();

                if let Some(store) = store_any.downcast_ref::<SparseSet<C>>() {
                    store.get(entity_id)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
