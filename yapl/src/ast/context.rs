use std::marker::PhantomData;

use crate::common::symbol::Symbol;

use crate::common::space::Id;

/// Basic struct that determines behavior of ast.
pub struct ContextPart<T> {
    _ty: PhantomData<T>,
}

pub struct Context<T> {
    data: Vec<ContextPart<T>>,
}

impl<T> ContextPart<T> {
    fn _set_meaning(&mut self, _symbol: Symbol, _meaning: ()) -> ! {
        todo!()
    }
    /// `Context` determines meaning of each `Symbol`.
    fn _meaning(&self, _symbol: Symbol) -> Id {
        todo!()
    }
}

impl<T> Context<T> {
    /// Actual context is combination of sequence of `Context`s,
    ///     overlapping each over.
    /// This adds new layer on stack.
    pub fn push(&self, _cover: Self) -> Self {
        todo!()
    }

    pub fn set_meaning(&mut self, _symbol: Symbol, _meaning: ()) -> ! {
        todo!()
    }
    /// `Context` determines meaning of each `Symbol`.
    pub fn meaning(&self, _symbol: Symbol) -> Id {
        todo!()
    }
}

impl<T> Default for ContextPart<T> {
    fn default() -> Self {
        let _ty = Default::default();
        Self { _ty }
    }
}

impl<T> Clone for ContextPart<T> {
    fn clone(&self) -> Self {
        let _ty = self._ty;
        Self { _ty }
    }
}
