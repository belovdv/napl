use std::iter::Peekable;

use crate::common::error::{raise_error, Result};
use crate::common::location::Span;

use super::ast::Line;
use super::errors::WrongLineOffset;

pub fn parse_line_hierarchy<I>(lines: &mut Peekable<I>, offset: usize) -> Result<Vec<Line>>
where
    I: Iterator<Item = (usize, Line)>,
{
    let mut result = Vec::new();
    while let Some(line) = lines.peek() {
        match line {
            (of, _) if *of < offset => break,
            (of, _) if *of == offset => {
                let (_, mut line) = lines.next().unwrap();
                line.update(
                    parse_line_hierarchy(lines, offset + 3)?,
                    parse_line_hierarchy(lines, offset + 1)?,
                );
                result.push(line)
            }
            (of, l) => {
                let l_s = l.span().begin();
                let of = *of as usize;
                raise_error!(WrongLineOffset, Span::new(l_s, l_s.advanced(of)), of)
            }
        }
    }
    Ok(result)
}
