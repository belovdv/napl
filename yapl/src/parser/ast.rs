use super::symbol::BracketType;

use crate::common::file::Span;
use crate::common::symbol::Symbol;

/// Representation of parsed file.
#[derive(Debug, Default)]
pub struct File {
    pub roots: Vec<Line>,
}

/// Line in code and its descendants.
#[derive(Debug, Default)]
pub struct Line {
    pub sentence: Sentence,
    pub extension: Vec<Line>,
    pub content: Vec<Line>,
    pub num: u16,
}

impl Line {
    pub fn new(sentence: Sentence) -> Self {
        let num = sentence.span.begin.line;
        Self {
            sentence,
            extension: Default::default(),
            content: Default::default(),
            num,
        }
    }

    pub fn empty(&self) -> bool {
        self.sentence.statements.is_empty()
    }
}

/// Sequence of statements.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Sentence {
    pub statements: Vec<(Statement, Span)>,
    pub span: Span,
}

/// Basic block of grammar.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Statement {
    #[default]
    None,

    Bracket((BracketType, Vec<Sentence>)),
    // First should be a word, next may be number.
    Chain(Vec<Symbol>),
    Special(Symbol),
    Inner(Box<Statement>),
    LitString(String),
    LitInt(i64), // To be done: big integer.
                 // To be done: float (lit number - enum?).
}
