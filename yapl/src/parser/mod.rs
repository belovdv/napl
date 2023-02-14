// Basic definitions.
mod ast;
mod errors;
mod symbol;
// Parsing.
mod lexer;
mod lines;
mod tree;

use crate::common::error::Error;
use crate::common::location::{Context, HasSpan};

pub fn parse(file: &Context) -> Result<ast::File, Vec<Error>> {
    let lines = lines::parse(file.code())?;
    let file_span = file.span();
    match tree::parse_line_hierarchy(&mut lines.into_iter().peekable(), 0) {
        Ok(v) => Ok(ast::File::new(file, v, file_span)),
        Err(e) => Err(vec![e]),
    }
}
