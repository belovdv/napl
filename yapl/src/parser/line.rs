use super::ast::{Line, Sentence, Statement};
use super::stream::Stream;
use super::symbol::{self, BracketType, SymbolType};
use super::unit;

use crate::common::file::{Error, Position, Span};

pub struct Parser<'a> {
    chars: Stream<'a>,
    pos: Position,
}

macro_rules! wrap_unit {
    ($uf:ident, $s:ident, $stt:ident) => {
        unit::$uf(&mut $s.chars).map(|d| Statement::$stt(d))
    };
}

impl<'a> Parser<'a> {
    pub fn new(line: &'a str, line_num: u16) -> Self {
        Self {
            chars: Stream::<'a>::new(line),
            pos: Position::new(line_num, 0),
        }
    }

    pub fn parse(&mut self) -> Result<Option<(u8, Line)>, Error> {
        let offset_s = self.parse_whitespace();
        let shift = self.chars.taken() as u8;
        let offset = offset_s / 4;
        if offset * 4 != offset_s {
            return Err(Error::new(
                "offset is not divisible by 4".to_string(),
                Span::new_p(self.pos, shift),
            ));
        }
        self.pos.mov(shift);

        let mut statements = Vec::new();
        while let Some(&c) = self.chars.peek() {
            statements.push(self.parse_statement(c)?)
        }

        if statements.len() == 0 {
            return Ok(None);
        }
        let span = Span::new(statements[0].1.begin, statements.last().unwrap().1.end);
        Ok(Some((offset, Line::new(Sentence { statements, span }))))
    }

    fn parse_statement(&mut self, peek: char) -> Result<(Statement, Span), Error> {
        let statement = match SymbolType::from(peek) {
            SymbolType::NewLine | SymbolType::EOS => panic!("{:?}", peek),
            SymbolType::Dot | SymbolType::Comma | SymbolType::Other => {
                Err(format!("unexpected symbol {}", peek))
            }
            SymbolType::Whitespace(_) => {
                self.parse_whitespace();
                Ok(Default::default())
            }
            SymbolType::Quote => wrap_unit!(string, self, LitString),
            SymbolType::Letter(_) => wrap_unit!(chain, self, Chain),
            SymbolType::Digit(_) => wrap_unit!(int, self, LitInt),
            SymbolType::Special(_) => wrap_unit!(special, self, Special),
            SymbolType::Inner => self.parse_inner(),
            SymbolType::Bracket(bt, open) => self.parse_bracket(bt, open),
        };
        let size = self.chars.taken() as u8;
        let span = Span::new_p(self.pos, size);
        self.pos.mov(size);
        match statement {
            Ok(st) => Ok((st, span)),
            Err(e) => Err(Error::new(e, span)),
        }
    }

    fn parse_whitespace(&mut self) -> u8 {
        let mut offset = 0;
        loop {
            match SymbolType::from(self.chars.peek().map(|&c| c)) {
                SymbolType::Whitespace(_) => offset += symbol::offset(self.chars.next()).unwrap(),
                SymbolType::NewLine => panic!(),
                _ => return offset,
            }
        }
    }

    fn parse_inner(&mut self) -> Result<Statement, String> {
        self.chars.next().unwrap();
        match self.chars.next() {
            Some(' ') => {
                // Skip comment.
                while let Some(_) = self.chars.next() {}
                Ok(Statement::None)
            }
            Some(_) => Err(format!("expected comment")),
            None => Err(format!("`inner` on the end of the line")),
        }
    }

    fn parse_bracket(&mut self, t: BracketType, is_open: bool) -> Result<Statement, String> {
        todo!()
    }
}
