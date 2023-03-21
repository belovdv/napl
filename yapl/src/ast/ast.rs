use crate::common::location::Span;
use crate::common::other::BracketType;
use crate::common::symbol::Symbol;

use super::context::ContextPart;
use super::{builtin, project};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, derive_new::new)]
pub struct Line {
    pub sent: Sent,
    pub extension: Vec<Line>,
    pub block: Vec<Line>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, derive_new::new)]
pub struct Sent {
    pub chains: Vec<Chain>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, derive_new::new)]
pub struct Chain {
    pub chain: Vec<(Expr, Span)>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, derive_new::new)]
pub enum Expr {
    Inner,
    Symbol(Symbol),
    Bracket { ty: BracketType, inner: Vec<Sent> },
    LitStr(String),
    LitInt(i64),
}
