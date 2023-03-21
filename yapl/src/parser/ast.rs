use crate::common::location::{self, implement_has_span, Span};
use crate::common::other::BracketType;
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

#[derive(Debug, PartialEq, Serialize, Deserialize, derive_new::new)]
pub struct Line {
    pub sent: Sent,
    // Defaults are initialized in `fn update`.
    #[new(default)]
    pub extension: Vec<Line>,
    #[new(default)]
    pub block: Vec<Line>,
    #[new(default)]
    pub span: Span, // Starts from `sent`. Contains all sub lines.
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sent {
    pub chains: Vec<Chain>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Chain {
    pub chain: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, derive_new::new)]
pub struct Expr {
    pub expr: ExprT,
    pub span: Span,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, derive_new::new)]
pub enum ExprT {
    Dot,
    Symbol(Symbol),
    Bracket { ty: BracketType, inner: Vec<Sent> },
    LitStr(String),
    LitInt(i64),
}

implement_has_span!(Expr, Sent, Line);

impl Line {
    pub fn update(&mut self, extension: Vec<Line>, block: Vec<Line>) {
        self.extension = extension;
        self.block = block;
        self.span = match (self.extension.last(), self.block.last()) {
            (Some(last), None) | (_, Some(last)) => self.span + last.span,
            (None, None) => self.sent.span,
        }
    }
}

impl Chain {
    // Expects non-empty vec.
    pub fn new(chain: Vec<Expr>) -> Self {
        let span = chain.first().unwrap().span + chain.last().unwrap().span;
        Self { chain, span }
    }
}

impl Sent {
    // Expects non-empty vec.
    pub fn new(chains: Vec<Chain>) -> Self {
        let span = chains.first().unwrap().span + chains.last().unwrap().span;
        Self { chains, span }
    }
}
