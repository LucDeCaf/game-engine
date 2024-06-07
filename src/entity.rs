use crate::{app::App, traits::Component};

pub struct EntityBuilder<'a> {
    entity_id: usize,
    app: &'a mut App,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(entity_id: usize, app: &'a mut App) -> Self {
        Self { entity_id, app }
    }

    pub fn add_component<C: Component>(self, component: C) -> Self {
        self.app.add_component(self.entity_id, component);
        self
    }

    pub fn id(&self) -> usize {
        self.entity_id
    }
}
