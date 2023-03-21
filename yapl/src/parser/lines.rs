use std::iter::Peekable;
use std::vec;

use super::ast::{Chain, Expr, ExprT, Line, Sent};
use super::errors::{
    ClosedBracket, ClosingBracketNotFound, NewLineOnFileEnd, UnexpectedEOL, UnexpectedToken,
    WrongLineOffset,
};
use super::lexer::{Lexer, Token};
use super::symbol::offset;

use crate::common::error::{raise_error, Error};
use crate::common::location::Span;
use crate::common::other::BracketType;

// To be done: fix risen code complexity.

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
            // To be done: remove allocation.
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
            Ok(s) => result.push((of, Line::new(s))),
            Err(e) => errors.push(e),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(result)
}

type Tokens<'a> = Peekable<std::vec::IntoIter<(Token, Span)>>;

fn parse_line(tokens: &mut Tokens) -> Result<Sent, Error> {
    debug_assert!(!matches!(tokens.peek().unwrap(), (Token::Whitespace(_), _)));

    let mut chains = Vec::new();
    while let Some((t, span)) = tokens.peek() {
        match t {
            Token::Whitespace(_) => {
                tokens.next();
            }
            Token::Comma => raise_error!(UnexpectedToken, *span,),
            Token::Bracket(_, false) => raise_error!(ClosedBracket, *span,),
            _ => chains.push(parse_chain(tokens)?),
        }
    }

    Ok(Sent::new(chains))
}

// Expects not `Comma`, "closing bracket" or `Whitespace` at the beginning. Doesn't consume end.
fn parse_chain(tokens: &mut Tokens) -> Result<Chain, Error> {
    let mut chain = Vec::new();
    while let Some((t, span)) = tokens.next() {
        chain.push(match t {
            Token::Dot => Expr::new(ExprT::new_dot(), span),
            Token::LitInt(i) => Expr::new(ExprT::new_lit_int(i), span),
            Token::LitStr(s) => Expr::new(ExprT::new_lit_str(s), span),
            Token::Bracket(bt, true) => parse_bracket(tokens, bt, span)?,
            Token::Word(s) | Token::Special(s) => Expr::new(ExprT::new_symbol(s), span),
            _ => raise_error!(UnexpectedToken, span,),
        });
        match tokens.peek().map(|(t, _)| t) {
            // `. ` starts comment. Consume iterator.
            Some(Token::Whitespace(_)) if matches!(chain.last().unwrap().expr, ExprT::Dot) => {
                tokens.count();
            }
            Some(Token::Comma) | Some(Token::Whitespace(_)) => break,
            Some(Token::Bracket(_, false)) => break,
            _ => {}
        };
    }
    Ok(Chain::new(chain))
}

// Expects first bracket to be already consumed.
fn parse_bracket(tokens: &mut Tokens, ty: BracketType, from: Span) -> Result<Expr, Error> {
    let mut to = from;
    let mut sent = Vec::new();
    let mut inner = Vec::new();
    while let Some((token, span)) = tokens.peek() {
        to = *span;
        match token {
            Token::Whitespace(_) => {
                tokens.next();
            }
            Token::Comma => {
                inner.push(Sent::new(sent));
                sent = Vec::new()
            }
            Token::Bracket(bt, false) if *bt == ty => {
                tokens.next();
                if sent.len() > 0 {
                    inner.push(Sent::new(sent));
                }
                return Ok(Expr::new(ExprT::Bracket { ty, inner }, from + to));
            }
            Token::NewLine => raise_error!(UnexpectedEOL, *span,),
            _ => sent.push(parse_chain(tokens)?),
        }
    }
    raise_error!(ClosingBracketNotFound, from + to,)
}
