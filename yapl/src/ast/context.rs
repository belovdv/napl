use std::collections::HashMap;
use std::marker::PhantomData;

use crate::common::symbol::Symbol;

use crate::common::space::{Id, Space};

/// Basic struct that determines behavior of ast.
pub struct Context {
    layers: Vec<ContextPart>,
}

/// Basic struct that determines behavior of ast.
#[derive(Default, Clone)]
pub struct ContextPart {
    context: HashMap<Symbol, Id>,
}

impl ContextPart {
    fn set_meaning(&mut self, symbol: Symbol, meaning: Id) {
        self.context.insert(symbol, meaning);
    }
    /// `Context` determines meaning of each `Symbol`.
    fn meaning(&self, symbol: Symbol) -> Option<Id> {
        self.context.get(&symbol).map(|&i| i)
    }
}

impl Context {
    /// Actual context is combination of sequence of `Context`s,
    ///     overlapping each over.
    /// This adds new layer on stack.
    pub fn push(&mut self, cover: ContextPart) {
        self.layers.push(cover)
    }
    pub fn pop(&mut self) {
        self.layers.pop();
    }

    pub fn set_meaning(&mut self, symbol: Symbol, meaning: Id) {
        // This should be called only with layers in it.
        self.layers.last_mut().unwrap().set_meaning(symbol, meaning)
    }
    /// `Context` determines meaning of each `Symbol`.
    pub fn meaning(&self, symbol: Symbol) -> Option<Id> {
        self.layers.iter().rev().find_map(|s| s.meaning(symbol))
    }
}
