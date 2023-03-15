use super::{Function, Instance, Set};

macro_rules! space_with_id {
    ($Space:ident, $Id:ident, $Type:ty) => {
        pub type $Space = crate::common::space::Space<$Type>;

        #[derive(Debug, Clone, Copy)]
        pub struct $Id(crate::common::space::Id);

        impl $Id {
            pub fn get<'space>(&self, space: &'space $Space) -> &'space $Type {
                space.get(self.0)
            }

            pub fn new(space: &mut $Space, value: $Type) -> Self {
                Self(space.insert(value))
            }
        }
    };
}

space_with_id!(SpaceI, ObjI, Instance);
space_with_id!(SpaceF, ObjF, Function);
space_with_id!(SpaceS, ObjS, Set);
