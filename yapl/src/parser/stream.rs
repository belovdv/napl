use std::iter::Peekable;
use std::str::Chars;

#[derive(derive_new::new, getset::Getters)]
pub struct Stream<'a> {
    chars: Peekable<Chars<'a>>,
    #[getset(get = "pub")]
    #[new(value = "0")]
    pos: usize,
}

impl<'a> Stream<'a> {
    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
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

impl<'a> From<&'a str> for Stream<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value.chars().peekable())
    }
}
