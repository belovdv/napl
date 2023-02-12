use std::iter::Peekable;

use crate::common::error::Result;

use super::ast::Line;
use super::errors::ErrorSimple;

pub fn parse_line_hierarchy<I>(lines: &mut Peekable<I>, offset: u8) -> Result<Vec<Line>>
where
    I: Iterator<Item = (u8, Line)>,
{
    let mut result = Vec::new();
    while let Some(line) = lines.peek() {
        match line {
            (of, _) if *of < offset => break,
            (of, _) if *of == offset => {
                let (_, mut line) = lines.next().unwrap();
                line.set_extension(parse_line_hierarchy(lines, offset + 3)?);
                line.set_block(parse_line_hierarchy(lines, offset + 1)?);
                result.push(line)
            }
            (_, l) => return Err(err("wrong offset".to_string(), l)),
        }
    }
    Ok(result)
}

fn err(message: String, line: &Line) -> Box<ErrorSimple> {
    // let len = line.sentence.statements.last().map(|(_, s)| s.end.offset);
    Box::new(ErrorSimple::new(message, Default::default()))
}
