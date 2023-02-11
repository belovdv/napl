// Definitions.
mod ast;
mod symbol;
// Chars wrapper.
mod stream;
// Parse unit statements.
mod unit;
// Parse single line.
mod line;
// Build file tree from line offsets.
mod tree;

use crate::common::file::{Context, Error};

pub fn parse(file: &Context) -> Result<ast::File, Vec<Error>> {
    let mut lines = Vec::new();
    let mut errors = Vec::new();
    file.lines
        .iter()
        .enumerate()
        .map(|(n, l)| line::Parser::new(l, n as u16).parse())
        .for_each(|r| match r {
            Ok(v) => lines.push(v),
            Err(e) => errors.push(e),
        });
    if !errors.is_empty() {
        return Err(errors);
    }

    match tree::parse_line_hierarchy(&mut lines.into_iter().flatten().peekable(), 0) {
        Ok(v) => Ok(ast::File { roots: v }),
        Err(e) => Err(vec![e]),
    }
}
