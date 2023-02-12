// Basic definitions.
mod ast;
mod errors;
mod stream;
mod symbol;
// Parsing.
mod line;
mod tree;
mod unit;

use crate::common::error::Error;
use crate::common::location::{Context, HasSpan};

pub fn parse(file: Context) -> Result<ast::File, Vec<Error>> {
    let lines = line::parse(file.code())?;
    let file_span = file.span();
    match tree::parse_line_hierarchy(&mut lines.into_iter().peekable(), 0) {
        Ok(v) => Ok(ast::File::new(file, v, file_span)),
        Err(e) => Err(vec![e]),
    }
}
