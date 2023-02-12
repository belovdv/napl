use super::symbol::BracketType;

use crate::common::location::{implement_has_span, Context, Span};
use crate::common::symbol::Symbol;

use serde::{Deserialize, Serialize};

#[derive(derive_new::new, getset::Getters)]
pub struct File {
    #[getset(get = "pub")]
    context: Context,
    #[getset(get = "pub")]
    roots: Vec<Line>,
    #[getset(get = "pub")]
    span: Span,
}

#[derive(Debug, PartialEq, derive_new::new, getset::CopyGetters, Serialize, Deserialize)]
pub struct Line {
    sent: Sent,
    #[new(default)]
    extension: Vec<Line>,
    #[new(default)]
    block: Vec<Line>,
    #[getset(get_copy = "pub")]
    span: Span, // Contains all sub lines.
}

impl Line {
    pub fn update(&mut self, extension: Vec<Line>, block: Vec<Line>) {
        self.extension = extension;
        self.block = block;
        self.span = match (self.extension.last(), self.block.last()) {
            (Some(last), None) | (_, Some(last)) => Span::new_contained(self.span, last.span),
            (None, None) => self.span,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sent {
    sent: Vec<Expr>,
    span: Span,
}

impl Sent {
    pub fn new(sent: Vec<Expr>) -> Option<Self> {
        if let (Some(first), Some(last)) = (sent.first(), sent.last()) {
            let span = Span::new_contained(first.span, last.span);
            return Some(Self { sent, span });
        };
        None
    }
}

#[derive(Debug, PartialEq, derive_new::new, Serialize, Deserialize)]
pub struct Expr {
    expr: ExprT,
    span: Span,
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
expr_new!(new_ls, LitS, val: String);
expr_new!(new_li, LitI, val: i64);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ExprT {
    Inner(Box<Expr>),
    Special(Symbol),
    Chain(Vec<Symbol>),
    Bracket(BracketType, Vec<Sent>),
    LitS(String),
    LitI(i64),
}

implement_has_span!(Expr, Sent, Line, File);
