mod types;

use crate::common::file::{Error, Position, Span};

use types::TokenType;

pub struct Token {
    pub ty: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(ty: TokenType, span: Span) -> Self {
        Self { ty, span }
    }
}

pub fn tokenize(code: &str) -> Result<Vec<Vec<Token>>, Vec<Error>> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    for (number_line, line) in code.lines().enumerate() {
        let mut chars = line.chars().peekable();
        let mut line = Vec::new();
        let mut pos = Position {
            line: number_line as u16,
            offset: 0,
        };
        while let Some(..) = chars.peek() {
            match TokenType::take(&mut chars, pos) {
                Ok((token, len)) => {
                    pos.offset += len;
                    line.push(Token::new(token, Span::new_p(pos, len)))
                }
                Err(e) => {
                    errors.push(e);
                    break;
                }
            }
        }
        tokens.push(line)
    }

    if errors.len() > 0 {
        return Err(errors);
    }
    Ok(tokens)
}
