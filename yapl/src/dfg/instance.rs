use super::{ObjF, ObjS, Space};

pub enum Instance {
    From(ObjS),                             // Input of func implementations.
    Mapped { from: Vec<ObjS>, with: ObjF }, // Result of called func.

    // Builtin: instance of builtin set.
    Any,
    Unit,
    Integer,
    Unsigned,
    AnySet,
    AnySetSeq,
    AnyFunc,
}

impl Instance {
    pub fn get_set(&self, space: &Space) -> ObjS {
        match self {
            Instance::From(s) => *s,
            Instance::Mapped { with, .. } => with.get(&space.f).get_output(space),
            Instance::Any => space.b_s.any,
            Instance::Unit => space.b_s.unit,
            Instance::Integer => space.b_s.integer,
            Instance::Unsigned => space.b_s.unsigned,
            Instance::AnySet => space.b_s.any_set,
            Instance::AnySetSeq => space.b_s.any_set_seq,
            Instance::AnyFunc => space.b_s.any_func,
        }
    }
}
