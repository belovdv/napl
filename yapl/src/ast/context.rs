use crate::common::symbol::Symbol;

use crate::common::space::Id;

/// Basic struct that determines behavior of ast.
pub struct Context;

impl Context {
    /// Actual context is combination of sequence of `Context`s,
    ///     overlapping each over.
    /// This adds new layer on stack.
    pub fn push(&self, _cover: &Self) -> Self {
        todo!()
    }
    pub fn pop() -> ! {
        todo!()
    }

    /// `Context` determines meaning of each `Symbol`.
    pub fn meaning(&self, _symbol: Symbol) -> Id {
        todo!()
    }
    pub fn set_meaning(&mut self, _symbol: Symbol, _meaning: ()) -> ! {
        todo!()
    }
}
