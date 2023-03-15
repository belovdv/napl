use super::{Instance, ObjI, ObjS, Set, SpaceI, SpaceS};

// To be done: references to unit may cause circular dependence.

impl BuiltinS {
    pub fn new(space_s: &mut SpaceS) -> Self {
        Self {
            any: ObjS::new(space_s, Set::Any),
            unit: ObjS::new(space_s, Set::Unit),
            integer: ObjS::new(space_s, Set::Integer),
            unsigned: ObjS::new(space_s, Set::Unsigned),
            any_set: ObjS::new(space_s, Set::AnySet),
            any_set_seq: ObjS::new(space_s, Set::AnySeq),
            any_func: ObjS::new(space_s, Set::AnyFunc),
        }
    }
}

impl BuiltinI {
    pub fn new(space_i: &mut SpaceI) -> Self {
        Self {
            any: ObjI::new(space_i, Instance::Any),
            unit_from: ObjI::new(space_i, Instance::Unit),
            unit_to: ObjI::new(space_i, Instance::Unit),
            integer: ObjI::new(space_i, Instance::Integer),
            unsigned: ObjI::new(space_i, Instance::Unsigned),
            any_set: ObjI::new(space_i, Instance::AnySet),
            any_set_seq: ObjI::new(space_i, Instance::AnySetSeq),
            any_func: ObjI::new(space_i, Instance::AnyFunc),
        }
    }
}

pub struct BuiltinS {
    pub any: ObjS,
    pub unit: ObjS,
    pub integer: ObjS,
    pub unsigned: ObjS,
    pub any_set: ObjS,
    pub any_set_seq: ObjS,
    pub any_func: ObjS,
}

pub struct BuiltinI {
    pub any: ObjI,
    pub unit_from: ObjI,
    pub unit_to: ObjI,
    pub integer: ObjI,
    pub unsigned: ObjI,
    pub any_set: ObjI,
    pub any_set_seq: ObjI,
    pub any_func: ObjI,
}
