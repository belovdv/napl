use std::iter::Peekable;

use crate::common::file::{Error, Span};

use super::ast::Line;

pub fn parse_line_hierarchy<I>(lines: &mut Peekable<I>, offset: u8) -> Result<Vec<Line>, Error>
where
    I: Iterator<Item = (u8, Line)>,
{
    let mut result = Vec::new();
    while let Some(line) = lines.peek() {
        match line {
            (of, _) if *of < offset => break,
            (of, _) if *of == offset => {
                let (_, mut line) = lines.next().unwrap();
                line.extension = parse_line_hierarchy(lines, offset + 2)?;
                line.content = parse_line_hierarchy(lines, offset + 1)?;
                result.push(line)
            }
            (_, l) => return Err(err("wrong offset".to_string(), l)),
        }
    }
    Ok(result)
}

fn err(message: String, line: &Line) -> Error {
    let len = line.sentence.statements.last().map(|(_, s)| s.end.offset);
    Error::new(message, Span::new_s(line.num, 0, len.unwrap_or(0)))
}
