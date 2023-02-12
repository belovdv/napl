use super::ast::{Expr, Line, Sent};
use super::errors::{
    ClosingBracket, EmptyBracketPart, UnsupportedSymbol, WrongBracket, WrongLineOffset,
};
use super::stream::Stream;
use super::symbol::{self, BracketType, SymbolType};
use super::unit;

use crate::common::error::{raise_error, Error};

pub fn parse(line: &str) -> Result<Vec<(usize, Line)>, Vec<Error>> {
    let mut result = Vec::new();
    let mut errors = Vec::new();
    let mut pos = 0;
    for line in line.lines() {
        match parse_line(&mut Stream::new(line.chars().peekable(), pos)) {
            Ok(Some(r)) => result.push(r),
            Ok(None) => {}
            Err(e) => errors.push(e),
        }
        pos += line.len() + 1;
    }

    if errors.len() != 0 {
        return Err(errors);
    }
    Ok(result)
}

pub fn parse_line(stream: &mut Stream) -> Result<Option<(usize, Line)>, Error> {
    let begin = stream.pos();
    let offset = match parse_whitespace(stream) {
        Some(o) => match symbol::offset(o) {
            Some(offset) => offset,
            None => raise_error!(WrongLineOffset, stream.span(begin), o),
        },
        None => return Ok(None),
    };

    let mut sent = Vec::new();
    while let Some(_) = stream.peek() {
        let Some(next) = parse_statement(stream)? else {
                break;
            };
        sent.push(next);
    }

    match Sent::new(sent) {
        Some(s) => Ok(Some((offset, Line::new(s, stream.span(begin))))),
        None => Ok(None),
    }
}

macro_rules! wrap_unit {
    ($stream:ident, $begin:ident, $uf:ident, $stt:ident) => {
        Ok(Some(
            unit::$uf($stream).map(|d| Expr::$stt(d, $stream.span($begin)))?,
        ))
    };
}

fn parse_statement(stream: &mut Stream) -> Result<Option<Expr>, Error> {
    parse_whitespace(stream);
    let begin = stream.pos();
    match SymbolType::from(stream.peek().map(|&c| c)) {
        SymbolType::Whitespace(_) | SymbolType::NewLine => panic!(),
        SymbolType::EOS => Ok(None),
        SymbolType::Dot => parse_inner(stream),
        SymbolType::Comma => {
            raise_error!(UnsupportedSymbol, stream.span(begin),)
        }
        SymbolType::Quote => wrap_unit!(stream, begin, string, new_ls),
        SymbolType::Letter(_) => wrap_unit!(stream, begin, chain, new_c),
        SymbolType::Digit(_) => wrap_unit!(stream, begin, int, new_li),
        SymbolType::Special(_) => wrap_unit!(stream, begin, special, new_s),
        SymbolType::Bracket(t, true) => Ok(Some(parse_bracket(stream, t)?)),
        SymbolType::Bracket(_, false) => raise_error!(WrongBracket, stream.span(begin),),
        SymbolType::Other => raise_error!(UnsupportedSymbol, stream.span(begin),),
    }
}

// Returns `None` if found `NewLine`.
// Returns logical offset.
fn parse_whitespace(stream: &mut Stream) -> Option<usize> {
    let mut offset = 0;
    loop {
        match SymbolType::from(stream.peek().map(|&c| c)) {
            SymbolType::Whitespace(of) => {
                offset += of as usize;
                stream.next().unwrap();
            }
            SymbolType::NewLine | SymbolType::EOS => return None,
            _ => return Some(offset),
        }
    }
}

fn parse_inner(stream: &mut Stream) -> Result<Option<Expr>, Error> {
    let begin = stream.pos();
    stream.next().unwrap();
    match stream.peek() {
        // It is comment.
        Some(' ') => {
            while matches!(stream.peek(), Some(c) if *c != '\n') {
                stream.next().unwrap();
            }
            Ok(None)
        }
        _ => raise_error!(UnsupportedSymbol, stream.span(begin),),
    }
}

fn parse_bracket(stream: &mut Stream, t: BracketType) -> Result<Expr, Error> {
    // To be done: rewrite.
    let begin = stream.pos();
    stream.next().unwrap();
    parse_whitespace(stream);
    match SymbolType::from(stream.peek().map(|&c| c)) {
        SymbolType::Bracket(tt, false) if tt == t => {
            stream.next().unwrap();
            return Ok(Expr::new_b(t, Vec::new(), stream.span(begin)));
        }
        _ => {}
    }

    let mut sentences = Vec::new();
    let mut sent = Vec::new();
    while let Some(_) = stream.peek() {
        let Some(next) = parse_statement(stream)? else {
            break;
        };
        sent.push(next);
        parse_whitespace(stream);
        match SymbolType::from(stream.peek().map(|&c| c)) {
            SymbolType::Comma => {
                match Sent::new(sent) {
                    Some(s) => sentences.push(s),
                    None => raise_error!(EmptyBracketPart, stream.span(begin),),
                }
                sent = Vec::new()
            }
            SymbolType::Bracket(tt, false) if tt == t => {
                stream.next().unwrap();
                match Sent::new(sent) {
                    Some(s) => sentences.push(s),
                    None => raise_error!(EmptyBracketPart, stream.span(begin),),
                }
                return Ok(Expr::new_b(t, sentences, stream.span(begin)));
            }
            _ => {}
        }
    }
    raise_error!(ClosingBracket, stream.span(begin),)
}
