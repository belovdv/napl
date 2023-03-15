mod builtin;
mod function;
mod instance;
mod set;
mod space;

pub use builtin::{BuiltinI, BuiltinS};
pub use function::Function;
pub use instance::Instance;
pub use set::Set;
pub use space::{ObjF, ObjI, ObjS, SpaceF, SpaceI, SpaceS};

pub struct Space {
    s: SpaceS,
    f: SpaceF,
    i: SpaceI,

    b_s: BuiltinS,
    b_i: BuiltinI,
}

impl Space {
    pub fn new() -> Self {
        let mut s = SpaceS::default();
        let mut f = SpaceF::default();
        let mut i = SpaceI::default();
        let b_s = BuiltinS::new(&mut s);
        let b_i = BuiltinI::new(&mut i);
        Self { s, f, i, b_s, b_i }
    }
}

impl ObjI {
    pub fn get_set(&self, space: &Space) -> ObjS {
        match self.get(&space.i) {
            Instance::From(s) => *s,
            Instance::Mapped { with, .. } => with.get_output(space),

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

impl ObjF {
    pub fn get_input(&self, space: &Space) -> Vec<ObjS> {
        match self.get(&space.f) {
            Function::Unit => vec![space.b_s.any],
            Function::Map { input, f } => vec![space.b_s.any_set_seq, space.b_s.any_func],
            Function::Fold { input, f } => vec![space.b_s.any_set_seq, space.b_s.any_func],
            Function::Defined { input, .. } => input.iter().map(|i| i.get_set(space)).collect(),
        }
    }

    pub fn get_output(&self, space: &Space) -> ObjS {
        self.get(&space.f).get_output(space)
    }
}
