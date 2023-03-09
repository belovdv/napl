use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Id {
    // To be done: 'space.
    id: u32,
    // To be done: return `T`.
    // _ty: PhantomData<T>,
}

pub struct Space<T> {
    data: Vec<T>,
}

impl<T> Space<T> {
    pub fn get(&self, id: Id) -> &T {
        &self.data[id.id as usize]
    }
    // pub fn get_mut(&mut self, id: Id) -> &mut T {
    //     &mut self.data[id.id as usize]
    // }

    // To be done: erase doesn't exist.
    pub fn insert(&mut self, t: T) -> Id {
        let id = self.data.len() as u32;
        self.data.push(t);
        Id { id }
    }
}

impl<T: std::cmp::PartialEq> Space<T> {
    pub fn insert_unique(&mut self, t: T) -> Id {
        let old = self.data.iter().position(|v| *v == t);
        if let Some(pos) = old {
            return Id { id: pos as u32 };
        }
        self.insert(t)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut space: Space<String> = Default::default();

        let abc = space.insert("abc".to_string());
        let bcd = space.insert("bcd".to_string());
        let cda = space.insert("cda".to_string());
        let bcd_c = bcd;

        assert_eq!(space.get(abc), "abc");
        assert_eq!(space.get(bcd), space.get(bcd_c));
        assert_eq!(space.get(cda), "cda");
    }
}
