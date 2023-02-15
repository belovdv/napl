pub struct Id {
    // To be done: 'space.
    id: u32,
}

pub struct Space<T> {
    data: Vec<T>,
}

impl<T> Space<T> {
    pub fn get(&self, id: Id) -> &T {
        &self.data[id.id as usize]
    }
    pub fn get_mut(&mut self, id: Id) -> &mut T {
        &mut self.data[id.id as usize]
    }

    // There isn't erase.
    pub fn insert(&mut self, t: T) -> Id {
        let id = self.data.len() as u32;
        self.data.push(t);
        Id { id }
    }
}

impl<T> Default for Space<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
