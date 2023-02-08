use std::iter::Peekable;
use std::str::Chars;

use crate::common::file::{Error, Position, Span};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BracketType {
    Round,
    Square,
    Curly,
}

impl BracketType {
    fn get_type(sym: char) -> Option<(Self, bool)> {
        match sym {
            '(' => Some((BracketType::Round, true)),
            ')' => Some((BracketType::Round, false)),
            '[' => Some((BracketType::Square, true)),
            ']' => Some((BracketType::Square, false)),
            '{' => Some((BracketType::Curly, true)),
            '}' => Some((BracketType::Curly, false)),
            _ => None,
        }
    }
}

pub enum TokenType {
    String,
    Bracket(BracketType, bool), // Type, is_open.
    // To be done: some other special symbols.
    Word,
    Number,
    Special,
    Whitespace,
}

impl TokenType {
    // Assume `chars` is not empty.
    pub fn take(chars: &mut Peekable<Chars>, pos: Position) -> Result<(Self, u8), Error> {
        let next = chars.peek();
        match SymbolType::determine(next.map(|c| *c)) {
            SymbolType::EOS => panic!(),
            SymbolType::Other => Err(Error {
                message: "unsupported symbol".to_string(),
                span: Span::new_p(pos, 1),
            }),
            SymbolType::Letter(_) => Ok((
                Self::Word,
                Self::take_collect(chars, |s| s.is_ascii_alphanumeric()),
            )),
            SymbolType::Digit(_) => Ok((
                Self::Number,
                Self::take_collect(chars, |s| s.is_ascii_alphanumeric()),
            )),
            SymbolType::Bracket(n) => {
                let (ty, open) = BracketType::get_type(n).unwrap();
                Ok((Self::Bracket(ty, open), 1))
            }
            SymbolType::Special(_) => Ok((
                Self::Special,
                Self::take_collect(chars, |s| {
                    SymbolType::determine(Some(s)) == SymbolType::Special(s)
                }),
            )),
            SymbolType::Whitespace(n) => Ok((Self::Whitespace, 1)),
        }
    }

    fn take_collect(chars: &mut Peekable<Chars>, check: fn(char) -> bool) -> u8 {
        let mut result = 0;
        while let Some(sym) = chars.peek() {
            if check(*sym) {
                result += 1;
                chars.next().unwrap();
            } else {
                break;
            }
        }
        result
    }

    fn _check_number(_number: String) -> Result<Self, Error> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SymbolType {
    EOS,
    Other,
    Letter(char),
    Digit(char),
    Bracket(char),
    Special(char),
    Whitespace(char),
}

impl SymbolType {
    pub fn determine(sym: Option<char>) -> Self {
        match sym {
            Some(sym) if " \t".contains(sym) => Self::Whitespace(sym),
            Some(sym) if "()[]{}".contains(sym) => Self::Bracket(sym),
            Some(sym) if "<>+-=/\\\"`!#$^%&*".contains(sym) => Self::Special(sym),
            Some(sym) if sym.is_ascii_alphabetic() => Self::Letter(sym),
            Some(sym) if sym.is_ascii_digit() => Self::Digit(sym),
            Some(_) => Self::Other,
            None => Self::EOS,
        }
    }
}
