use std::iter::Peekable;
use std::str::Chars;

use crate::common::location::{Position, Span};

#[derive(derive_new::new, getset::CopyGetters)]
pub struct Stream<'a> {
    chars: Peekable<Chars<'a>>,
    #[getset(get_copy = "pub")]
    pos: usize,
}

impl<'a> Stream<'a> {
    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    pub fn span(&self, begin: usize) -> Span {
        Span::new(
            Position::new(begin).unwrap(),
            Position::new(self.pos).unwrap(),
        )
    }
}

impl<'a> Iterator for Stream<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.chars.next();
        if next.is_some() {
            self.pos += 1;
        }
        next
    }
}
