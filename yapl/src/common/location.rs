use std::fmt::Debug;
use std::path::{Path, PathBuf};

#[derive(getset::Getters)]
pub struct Context {
    path: PathBuf,
    #[getset(get = "pub")]
    code: String,
    lines: Vec<String>,
}

const MAX_FILE_SIZE: usize = 60000;

impl Context {
    pub fn new_read(path: PathBuf) -> Result<Self, String> {
        let code = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        if code.len() > MAX_FILE_SIZE {
            return Err(format!("to long file {:?}", path.as_os_str()));
        }
        let lines: Vec<_> = code.lines().map(|s| s.to_string()).collect();
        Ok(Self { path, code, lines })
    }

    pub fn get_path(&self) -> &Path {
        self.path.as_path()
    }
}

impl HasSpan for Context {
    // All file.
    fn span(&self) -> Span {
        Span::new(Default::default(), Position::new(self.code.len()).unwrap())
    }
}

// Note: don't forget, it has mean only in one `Context`.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Position {
    pos: u16,
}

impl Position {
    pub fn new(pos: usize) -> Option<Self> {
        match pos {
            pos if pos > MAX_FILE_SIZE => None,
            pos => Some(Self { pos: pos as u16 }),
        }
    }

    pub fn advance(&mut self, shift: usize) {
        self.pos += shift as u16
    }
    pub fn advanced(&self, shift: usize) -> Self {
        Position {
            pos: self.pos + shift as u16,
        }
    }

    // To be done: sublinear algorithm.
    // Note: don't forget, it has mean only in one `Context`.
    pub fn get_line_and_offset(&self, context: &Context) -> Option<(usize, usize)> {
        let mut pos = self.pos as usize;
        for (line_number, line) in context.lines.iter().enumerate() {
            if pos >= line.len() {
                pos -= line.len();
            } else {
                return Some((line_number, pos));
            }
        }
        None
    }
}

pub trait HasPosition {
    fn position(&self) -> Position;
}

// Note: don't forget, it has mean only in one `Context`.
#[derive(Default, Clone, Copy, PartialEq, getset::CopyGetters)]
pub struct Span {
    #[getset(get_copy = "pub")]
    begin: Position,
    #[getset(get_copy = "pub")]
    end: Position,
}

impl Span {
    pub fn new(begin: Position, end: Position) -> Self {
        assert!(begin <= end);
        Self { begin, end }
    }

    pub fn new_contained(first: Span, second: Span) -> Self {
        assert!(first.begin <= second.begin);
        assert!(first.end <= second.end);
        Self {
            begin: first.begin,
            end: second.end,
        }
    }

    pub fn contains(&self, inner: Span) -> bool {
        self.begin <= inner.begin && self.end >= inner.end
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Span({}, {})", self.begin.pos, self.end.pos))
    }
}

pub trait HasSpan {
    fn span(&self) -> Span;
}

// To be done: implemented as derive macro.
macro_rules! implement_has_span {
    ($($name:ident),*) => {
        $(impl crate::common::location::HasSpan for $name {
            fn span(&self) -> Span {
                self.span
            }
        })*
    };
}
pub(crate) use implement_has_span;
