use std::iter::Peekable;
use std::vec;

use super::ast::{Expr, Line, Sent};
use super::errors::{
    ClosedBracket, ClosingBracketNotFound, EmptyPartInBrackets, NewLineOnFileEnd, UnexpectedEOL,
    UnexpectedSymbol, UnexpectedToken, WrongLineOffset,
};
use super::lexer::{Lexer, Token};
use super::symbol::{offset, BracketType};

use crate::common::error::{raise_error, Error};
use crate::common::location::Span;
use crate::common::symbol::Symbol;

// To be done: fix risen (after fixing using slices) code complexity.

pub fn parse(line: &str) -> Result<Vec<(usize, Line)>, Vec<Error>> {
    // To be done: remove unnecessary allocations.
    let mut lines = Vec::new();
    let mut l_cur = Vec::new();
    let mut errors = Vec::new();
    for token in Lexer::new(line) {
        match token {
            Ok((Token::NewLine, _)) => {
                lines.push(l_cur);
                l_cur = Vec::new()
            }
            Ok((token, span)) => l_cur.push((token, span)),
            Err(e) => errors.push(e),
        }
    }
    if !l_cur.is_empty() {
        return Err(vec![Box::new(NewLineOnFileEnd::new(Default::default()))]);
    }

    let mut result = Vec::new();
    for mut line in lines.into_iter() {
        let (of, tokens) = match line.first().map(|i| i.clone()) {
            Some((Token::Whitespace(w), s)) if line.len() > 1 => match offset(w) {
                Some(of) => (of, line.drain(1..).collect()),
                None => {
                    errors.push(Box::new(WrongLineOffset::new(s, w)));
                    continue;
                }
            },
            Some((_, _)) if line.len() > 0 => (0, line),
            _ => continue,
        };
        match parse_line(&mut tokens.into_iter().peekable()) {
            Ok(Some(line)) => result.push((of, line)),
            Ok(None) => {}
            Err(e) => errors.push(e),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(result)
}

type Tokens<'a> = Peekable<std::vec::IntoIter<(Token, Span)>>;

pub fn parse_line(tokens: &mut Tokens) -> Result<Option<Line>, Error> {
    let mut sent = Vec::new();
    while let Some((token, span)) = tokens.next() {
        sent.push(parse_expr(tokens, token, span)?)
    }
    match Sent::new(sent.into_iter().flatten().collect()) {
        Some(sent) => Ok(Some(Line::new(sent))),
        None => Ok(None),
    }
}

fn parse_expr(tokens: &mut Tokens, token: Token, span: Span) -> Result<Option<Expr>, Error> {
    let result = match token {
        Token::Comma => raise_error!(UnexpectedSymbol, span, ','),
        Token::Bracket(_, false) => raise_error!(ClosedBracket, span,),
        Token::Dot => parse_inner(tokens, span)?,
        Token::Word(w) => Some(Expr::new_r(w, span)),
        Token::Bracket(bt, true) => Some(parse_bracket(tokens, bt, span)?),
        Token::Special(s) => Some(Expr::new_r(s, span)),
        Token::LitInt(li) => Some(Expr::new_li(li, span)),
        Token::LitStr(ls) => Some(Expr::new_ls(ls, span)),
        _ => None,
    };

    let Some(result) = result else {
        return Ok(None);
    };

    // To be done: this is wrong span.
    let (chain, span_end) = parse_chain(tokens, span)?;
    Ok(Some(match chain.len() {
        0 => result,
        _ => Expr::new_c(Box::new(result), chain, span + span_end),
    }))
}

fn parse_inner(tokens: &mut Tokens, begin: Span) -> Result<Option<Expr>, Error> {
    match tokens.next() {
        Some((Token::Whitespace(_), _)) => {
            while let Some(_) = tokens.next() {} // Comment - drain iterator.
            Ok(None)
        }
        Some((Token::Word(w), s)) => Ok(Some(Expr::new_i(w, begin + s))),
        Some((_, span)) => raise_error!(UnexpectedToken, span,),
        None => raise_error!(UnexpectedEOL, begin,),
    }
}

fn parse_chain(tokens: &mut Tokens, from: Span) -> Result<(Vec<Symbol>, Span), Error> {
    let mut chain = Vec::new();
    let mut to = from;
    while let Some((Token::Dot, _)) = tokens.peek() {
        let (_, span) = tokens.next().unwrap();
        chain.push(match tokens.next() {
            Some((Token::Word(w), span)) => {
                to = span;
                w
            }
            Some((_, span)) => raise_error!(UnexpectedToken, span,),
            None => raise_error!(UnexpectedEOL, span,),
        })
    }
    Ok((chain, to))
}

fn parse_bracket(tokens: &mut Tokens, bt: BracketType, from: Span) -> Result<Expr, Error> {
    let mut to = from;
    let mut expr = Vec::new();
    let mut sent = Vec::new();
    while let Some((token, span)) = tokens.next() {
        to = span;
        sent.push(match token {
            Token::Comma => {
                expr.push(match Sent::new(sent) {
                    Some(next) => next,
                    None => raise_error!(EmptyPartInBrackets, from + to,),
                });
                sent = Vec::new();
                continue;
            }
            Token::Bracket(t, false) if t == bt => {
                if !expr.is_empty() && sent.is_empty() {
                    raise_error!(EmptyPartInBrackets, from + to,)
                }
                match Sent::new(sent) {
                    Some(next) => expr.push(next),
                    None => {}
                };
                return Ok(Expr::new_b(bt, expr, from + to));
            }
            _ => match parse_expr(tokens, token, span)? {
                Some(next) => next,
                None => continue,
            },
        })
    }
    raise_error!(ClosingBracketNotFound, from + to,)
}
