use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Id {
    // To be done: 'space.
    id: u32,
    // To be done: return `T`.
    // _ty: PhantomData<T>,
}

pub struct Space<T: std::cmp::PartialEq> {
    data: Vec<T>,
}

impl<T: std::cmp::PartialEq> Space<T> {
    pub fn get(&self, id: Id) -> &T {
        &self.data[id.id as usize]
    }
    // pub fn get_mut(&mut self, id: Id) -> &mut T {
    //     &mut self.data[id.id as usize]
    // }

    // To be done: erase doesn't exist.
    pub fn insert(&mut self, t: T) -> Id {
        let old = self.data.iter().position(|v| *v == t);
        if let Some(pos) = old {
            return Id { id: pos as u32 };
        }
        let id = self.data.len() as u32;
        self.data.push(t);
        Id { id }
    }
}

impl Id {
    pub fn as_u32(&self) -> u32 {
        self.id
    }
}

impl<T: std::cmp::PartialEq> Default for Space<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Id").field("id", &self.id).finish()
    }
}

// impl<T> Clone for Id {
//     fn clone(&self) -> Self {
//         Self {
//             id: self.id.clone(),
//             _ty: self._ty.clone(),
//         }
//     }
// }
