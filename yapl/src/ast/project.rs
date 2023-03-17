use crate::common::Symbol;

// To be done: all of this is temporary replacement of `core`.

#[derive(Debug)]
pub struct Function {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub stmts: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    ConstantS(String),
    ConstantI(i64),

    Symbol(Symbol),
    Inner {
        from: Box<Statement>,
        with: Vec<Symbol>,
    },

    Mapped {
        func: Symbol, // To be done: not only operator.
        args: Vec<Statement>,
    },

    Set {
        ty: (),
        args: Vec<Symbol>,
        stmts: Vec<Statement>,
    },
}
