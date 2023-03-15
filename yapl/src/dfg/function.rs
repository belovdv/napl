use super::{ObjF, ObjI, ObjS, Set, Space};

pub enum Function {
    Unit,

    Map { input: ObjI, f: ObjF },
    Fold { input: ObjI, f: ObjF },

    Defined { input: Vec<ObjI>, output: ObjI },
}

impl Function {
    pub fn new_def(input: Vec<ObjI>, output: ObjI, space: &mut Space) -> ObjF {
        // assert(all dependency ways from output come to input)
        ObjF::new(&mut space.f, Self::Defined { input, output })
    }

    pub fn get_output(&self, space: &Space) -> ObjS {
        match self {
            Function::Unit => space.b_s.unit,
            Function::Map { f, .. } => f.get_output(space),
            Function::Fold { f, .. } => f.get_output(space),
            Function::Defined { output, .. } => output.get(&space.i).get_set(space),
        }
    }
}
