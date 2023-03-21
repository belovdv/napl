use crate::common::error::{raise_error, Result};
use crate::common::location::{Position, Span};
use crate::common::other::BracketType;
use crate::common::symbol::Symbol;

use super::errors::{ParseInt, UnexpectedEOS, UnexpectedSymbol, UnsupportedSymbol};
use super::symbol::SymbolType;

use std::iter::Peekable;
use std::str::Chars;

// To be done: fix risen complexity after error fixes.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Dot,
    Comma,
    NewLine,
    Bracket(BracketType, bool),
    Whitespace(usize),
    Special(Symbol),
    Word(Symbol),
    LitInt(i64),
    LitStr(String),
}

pub struct Lexer<'a> {
    stream: Stream<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        let stream = Stream::new(code.chars().peekable(), Default::default());
        Self { stream }
    }

    fn take(&mut self, begin: Position, start: char) -> Result<(Token, Span)> {
        let result = match SymbolType::from(start) {
            SymbolType::Dot => Token::Dot,
            SymbolType::Comma => Token::Comma,
            SymbolType::NewLine => Token::NewLine,
            SymbolType::Bracket(bt, open) => Token::Bracket(bt, open),
            SymbolType::Quote => string(&mut self.stream, begin)?,
            SymbolType::Letter(c) => word(&mut self.stream, begin, c)?,
            SymbolType::Digit(c) => number(&mut self.stream, begin, c)?,
            SymbolType::Special(c) => special(&mut self.stream, begin, c)?,
            SymbolType::Whitespace(w) => whitespace(&mut self.stream, begin, w)?,
            SymbolType::Other(s) => raise_error!(UnsupportedSymbol, self.stream.span(begin), s),
            SymbolType::EOS => raise_error!(UnexpectedEOS, self.stream.span(begin),),
        };
        Ok((result, self.stream.span(begin)))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(Token, Span)>;

    fn next(&mut self) -> Option<Self::Item> {
        let begin = self.stream.pos;
        match self.stream.next() {
            Some(c) => Some(self.take(begin, c)),
            None => None,
        }
    }
}

fn string(stream: &mut Stream, begin: Position) -> Result<Token> {
    let mut result = String::new();
    loop {
        match stream.next() {
            Some('\\') => result.push(match stream.next() {
                Some('\\') => '\\',
                Some('n') => '\n',
                Some('t') => '\t',
                Some('"') => '"',
                Some(c) => raise_error!(UnexpectedSymbol, stream.span(begin), c),
                None => raise_error!(UnexpectedEOS, stream.span(begin),),
            }),
            Some('"') => return Ok(Token::LitStr(result)),
            Some(c) => result.push(c),
            None => raise_error!(UnexpectedEOS, stream.span(begin),),
        }
    }
}

fn whitespace(stream: &mut Stream, begin: Position, first: usize) -> Result<Token> {
    let mut result = first;
    loop {
        match SymbolType::from(stream.chars.peek().map(|&c| c)) {
            SymbolType::Whitespace(w) => {
                result += w;
                stream.next().unwrap();
            }
            SymbolType::Other(s) => raise_error!(UnsupportedSymbol, stream.span(begin), s),
            _ => return Ok(Token::Whitespace(result)),
        }
    }
}

fn word(stream: &mut Stream, begin: Position, start: char) -> Result<Token> {
    let mut result = String::from(start);
    loop {
        match SymbolType::from(stream.chars.peek().map(|&c| c)) {
            SymbolType::Letter(_) | SymbolType::Digit(_) => result.push(stream.next().unwrap()),
            SymbolType::Other(s) => raise_error!(UnsupportedSymbol, stream.span(begin), s),
            _ => return Ok(Token::Word(Symbol::from(result))),
        }
    }
}

fn special(stream: &mut Stream, begin: Position, start: char) -> Result<Token> {
    let mut result = String::from(start);
    loop {
        match SymbolType::from(stream.chars.peek().map(|&c| c)) {
            SymbolType::Special(_) => result.push(stream.next().unwrap()),
            SymbolType::Other(s) => raise_error!(UnsupportedSymbol, stream.span(begin), s),
            _ => return Ok(Token::Special(Symbol::from(result))),
        }
    }
}

// To be done: parse other numbers. For now, `dot` is unexpected.
fn number(stream: &mut Stream, begin: Position, start: char) -> Result<Token> {
    let mut result = String::from(start);
    loop {
        match SymbolType::from(stream.chars.peek().map(|&c| c)) {
            SymbolType::Letter(_) | SymbolType::Digit(_) => result.push(stream.next().unwrap()),
            SymbolType::Other(s) => raise_error!(UnsupportedSymbol, stream.span(begin), s),
            SymbolType::Dot => raise_error!(UnexpectedSymbol, stream.span(begin), '.'),
            _ => match result.parse::<i64>() {
                Ok(r) => return Ok(Token::LitInt(r)),
                Err(_) => raise_error!(ParseInt, stream.span(begin), result),
            },
        }
    }
}

#[derive(derive_new::new)]
struct Stream<'a> {
    chars: Peekable<Chars<'a>>,
    pos: Position,
}

impl<'a> Stream<'a> {
    pub fn span(&self, begin: Position) -> Span {
        Span::new(begin, self.pos)
    }
}

impl<'a> Iterator for Stream<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.chars.next();
        if next.is_some() {
            self.pos.advance(1);
        }
        next
    }
}
