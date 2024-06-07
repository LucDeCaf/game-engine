#[derive(Debug)]
pub struct SparseSet<C> {
    sparse: Vec<Option<usize>>,
    dense: Vec<C>,
    entities: Vec<usize>,
}

impl<C> SparseSet<C> {
    pub fn remove(&mut self, entity_id: usize) -> Option<C> {
        if entity_id >= self.sparse.len() {
            return None;
        }

        match self.sparse[entity_id] {
            Some(index) => {
                let last_dense = self.dense.len() - 1;

                self.dense.swap(index, last_dense);
                self.entities.swap(index, last_dense);

                let removed = self.dense.pop();
                self.entities.pop();

                self.sparse[self.entities[index]] = Some(index);
                self.sparse[entity_id] = None;

                removed
            }
            None => None,
        }
    }

    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity_id: usize, component: C) {
        if entity_id >= self.sparse.len() {
            self.sparse.resize(entity_id + 1, None);
        }

        if let Some(index) = self.sparse[entity_id] {
            self.dense[index] = component;
        } else {
            self.sparse[entity_id] = Some(self.dense.len());
            self.dense.push(component);
            self.entities.push(entity_id);
        }
    }

    pub fn get(&self, entity_id: usize) -> Option<&C> {
        let possible_dense_index = self.sparse.get(entity_id)?;
        let dense_index = (*possible_dense_index)?;
        self.dense.get(dense_index)
    }

    pub fn get_mut(&mut self, entity_id: usize) -> Option<&mut C> {
        let possible_dense_index = self.sparse.get(entity_id)?;
        let dense_index = (*possible_dense_index)?;
        self.dense.get_mut(dense_index)
    }

    pub fn clear(&mut self) {
        self.sparse.clear();
        self.dense.clear();
        self.entities.clear();
    }
}
