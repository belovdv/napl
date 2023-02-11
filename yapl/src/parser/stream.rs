use std::iter::Peekable;
use std::str::Chars;

pub struct Stream<'a> {
    chars: Peekable<Chars<'a>>,
    taken: usize,
}

impl<'a> Stream<'a> {
    pub fn new(stream: &'a str) -> Self {
        Self::new_c(stream.chars().peekable())
    }
    pub fn new_c(chars: Peekable<Chars<'a>>) -> Self {
        Self { chars, taken: 0 }
    }

    pub fn taken(&mut self) -> usize {
        let result = self.taken;
        self.taken = 0;
        result
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
}

impl<'a> Iterator for Stream<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.chars.next();
        if next.is_some() {
            self.taken += 1;
        }
        next
    }
}

impl<'a> From<&'a str> for Stream<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}
