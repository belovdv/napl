// To be done: is pub necessary?
pub use super::symbol::BracketType;

use crate::common::location::{self, implement_has_span, Span};
use crate::common::symbol::Symbol;

use serde::{Deserialize, Serialize};

#[derive(derive_new::new, getset::Getters)]
pub struct File<'file> {
    #[getset(get = "pub")]
    context: &'file location::File,
    #[getset(get = "pub")]
    roots: Vec<Line>,
    #[getset(get = "pub")]
    span: Span,
}

// To be done: write template ast to not copy.

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub sent: Sent,
    pub extension: Vec<Line>,
    pub block: Vec<Line>,
    pub span: Span, // Starts from `sent`. Contains all sub lines.
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sent {
    pub sent: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, PartialEq, derive_new::new, Serialize, Deserialize)]
pub struct Expr {
    pub expr: ExprT,
    pub span: Span,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum ExprT {
    Inner(Symbol),
    Reference(Symbol),
    Chain { from: Box<Expr>, with: Vec<Symbol> },
    Bracket { ty: BracketType, inner: Vec<Sent> },
    LitStr(String),
    LitInt(i64),
}

implement_has_span!(Expr, Sent, Line);

impl Line {
    pub fn new(sent: Sent) -> Self {
        let span = sent.span;
        Self {
            sent,
            extension: Default::default(),
            block: Default::default(),
            span,
        }
    }

    pub fn update(&mut self, extension: Vec<Line>, block: Vec<Line>) {
        self.extension = extension;
        self.block = block;
        self.span = match (self.extension.last(), self.block.last()) {
            (Some(last), None) | (_, Some(last)) => self.span + last.span,
            (None, None) => self.span,
        }
    }
}

impl Sent {
    pub fn new(sent: Vec<Expr>) -> Option<Self> {
        if let (Some(first), Some(last)) = (sent.first(), sent.last()) {
            let span = first.span + last.span;
            return Some(Self { sent, span });
        };
        None
    }
}

macro_rules! expr_new {
    ($new_name:ident, $expr_t:ident, $($data:ident: $ty:ty),*) => {
        impl Expr {
            pub fn $new_name($($data: $ty),*, span: Span) -> Self {
                Self::new(ExprT::$expr_t($($data),*), span)
            }
        }
    };
}
macro_rules! expr_new_var {
    ($new_name:ident, $expr_t:ident, $($data:ident: $ty:ty),*) => {
        impl Expr {
            pub fn $new_name($($data: $ty),*, span: Span) -> Expr {
                Self::new(ExprT::$expr_t{ $($data),* }, span)
            }
        }
    };
}
expr_new!(new_i, Inner, inner: Symbol);
expr_new!(new_r, Reference, symbol: Symbol);
expr_new_var!(new_c, Chain, from: Box<Expr>, with: Vec<Symbol>);
expr_new_var!(new_b, Bracket, ty: BracketType, inner: Vec<Sent>);
expr_new!(new_ls, LitStr, val: String);
expr_new!(new_li, LitInt, val: i64);

impl std::fmt::Debug for ExprT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inner(arg0) => f.debug_tuple("Inner").field(arg0).finish(),
            // Self::Reference(arg0) => f.debug_tuple("Reference").field(arg0).finish(),
            Self::Reference(arg0) => f.write_fmt(format_args!("Reference({:?})", *arg0)),
            Self::Chain { from, with } => f
                .debug_struct("Chain")
                .field("from", from)
                .field("with", with)
                .finish(),
            Self::Bracket { ty, inner } => f
                .debug_struct("Bracket")
                .field("ty", ty)
                .field("inner", inner)
                .finish(),
            Self::LitStr(arg0) => f.debug_tuple("LitStr").field(arg0).finish(),
            Self::LitInt(arg0) => f.debug_tuple("LitInt").field(arg0).finish(),
        }
    }
}
