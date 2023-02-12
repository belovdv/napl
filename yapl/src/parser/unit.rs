// Unit functions expect they are called on correct place.

use crate::common::error::{raise_error, Result};
use crate::common::symbol::Symbol;

use super::errors::{
    ExpectedIdentifier, ExpectedWhitespace, LiteralString, ParseInt, UnsupportedSymbol,
};
use super::stream::Stream;
use super::symbol::SymbolType;

// To be done: somehow remove repeating code.

#[derive(PartialEq)]
enum SPS {
    None,
    Slash,
    Exit,
}
pub fn string(chars: &mut Stream) -> Result<String> {
    let begin = chars.pos();
    let mut result = String::new();
    let next = chars.next().unwrap();
    assert!(next == '"');
    let mut state = SPS::None;
    while state != SPS::Exit {
        let Some(next) = chars.next() else {
            raise_error!(LiteralString, chars.span(begin),);
        };
        match state {
            SPS::None => match next {
                '\\' => state = SPS::Slash,
                '"' => state = SPS::Exit,
                _ => result.push(next),
            },
            SPS::Slash => {
                match next {
                    '\\' | '"' => result.push(next),
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    _ => raise_error!(UnsupportedSymbol, chars.span(begin),),
                }
                state = SPS::None
            }
            SPS::Exit => panic!(),
        }
    }
    Ok(result)
}

pub fn chain(chars: &mut Stream) -> Result<Vec<Symbol>> {
    let begin = chars.pos();
    let mut result = Vec::new();
    let mut s = String::new();
    loop {
        match SymbolType::from(chars.peek().map(|&c| c)) {
            SymbolType::Dot => {
                if s.len() == 0 {
                    raise_error!(ExpectedIdentifier, chars.span(begin),);
                }
                result.push(Symbol::from(s));
                s = String::new();
                chars.next().unwrap();
            }
            SymbolType::Quote => {
                raise_error!(ExpectedWhitespace, chars.span(begin),)
            }
            SymbolType::Letter(_) | SymbolType::Digit(_) => s.push(chars.next().unwrap()),
            SymbolType::Other => raise_error!(UnsupportedSymbol, chars.span(begin),),
            _ => break,
        }
    }
    if s.len() == 0 {
        raise_error!(ExpectedIdentifier, chars.span(begin),)
    }
    result.push(Symbol::from(s));
    Ok(result)
}

pub fn special(chars: &mut Stream) -> Result<Symbol> {
    let begin = chars.pos();
    let mut result = String::new();
    loop {
        match SymbolType::from(chars.peek().map(|&c| c)) {
            SymbolType::Quote => {
                raise_error!(ExpectedWhitespace, chars.span(begin),)
            }
            SymbolType::Special(_) => result.push(chars.next().unwrap()),
            SymbolType::Other | SymbolType::Dot => {
                raise_error!(UnsupportedSymbol, chars.span(begin),)
            }
            _ => break,
        }
    }
    Ok(Symbol::from(result))
}

pub fn int(chars: &mut Stream) -> Result<i64> {
    let begin = chars.pos();
    let mut result = String::new();
    loop {
        match SymbolType::from(chars.peek().map(|&c| c)) {
            SymbolType::Quote => {
                raise_error!(ExpectedWhitespace, chars.span(begin),)
            }
            SymbolType::Digit(_) => result.push(chars.next().unwrap()),
            SymbolType::Other | SymbolType::Dot => {
                raise_error!(UnsupportedSymbol, chars.span(begin),)
            }
            _ => break,
        }
    }
    match result.parse::<i64>() {
        Ok(r) => Ok(r),
        Err(e) => Err(Box::new(ParseInt::new(chars.span(begin), e.to_string()))),
    }
}
