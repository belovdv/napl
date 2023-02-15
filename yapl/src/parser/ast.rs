use super::symbol::BracketType;

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

#[derive(Debug, PartialEq, getset::CopyGetters, Serialize, Deserialize)]
pub struct Line {
    sent: Sent,
    extension: Vec<Line>,
    block: Vec<Line>,
    #[getset(get_copy = "pub")]
    span: Span, // Starts from `sent`. Contains all sub lines.
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sent {
    sent: Vec<Expr>,
    span: Span,
}

#[derive(Debug, PartialEq, derive_new::new, Serialize, Deserialize)]
pub struct Expr {
    expr: ExprT,
    span: Span,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ExprT {
    Inner(Box<Expr>),
    Special(Symbol),
    Chain(Vec<Symbol>),
    Bracket(BracketType, Vec<Sent>),
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
expr_new!(new_i, Inner, inner: Box<Expr>);
expr_new!(new_s, Special, special: Symbol);
expr_new!(new_c, Chain, chain: Vec<Symbol>);
expr_new!(new_b, Bracket, ty: BracketType, parts: Vec<Sent>);
expr_new!(new_ls, LitStr, val: String);
expr_new!(new_li, LitInt, val: i64);
