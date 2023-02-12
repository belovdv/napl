// Basic definitions.
mod ast;
mod errors;
mod stream;
mod symbol;
// Parse unit statements.
mod unit;
// Parse single line.
mod line;
// Build file tree from line offsets.
mod tree;

use crate::common::error::Error;
use crate::common::location::Context;

pub fn parse(file: Context) -> Result<ast::File, Vec<Box<dyn Error>>> {
    let mut lines = Vec::new();
    let mut errors = Vec::new();
    file.code()
        .lines()
        .enumerate()
        .map(|(n, l)| line::Parser::new(l, n).parse())
        .for_each(|r| match r {
            Ok(v) => lines.push(v),
            Err(e) => errors.push(e),
        });
    if !errors.is_empty() {
        return Err(errors);
    }

    match tree::parse_line_hierarchy(&mut lines.into_iter().flatten().peekable(), 0) {
        Ok(v) => Ok(ast::File::new(file, v, Default::default())),
        Err(e) => Err(vec![e]),
    }
}
