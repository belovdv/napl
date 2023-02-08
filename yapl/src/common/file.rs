use std::path::PathBuf;

pub struct Context {
    pub file: PathBuf,
    pub lines: Vec<String>,
}

impl Context {
    fn new_read(file: PathBuf) -> Result<Self, Error> {
        match std::fs::read_to_string(&file) {
            Ok(s) => {
                let lines: Vec<_> = s.lines().map(|l| l.to_string()).collect();
                if lines.len() > 10000 {
                    Err(Error::new("too many lines".to_string(), Default::default()))
                } else if let Some(long) = lines.iter().position(|l| l.len() > 200) {
                    Err(Error::new(
                        "too long line".to_string(),
                        Span::new(long as u16, 0, lines[long].len() as u8),
                    ))
                } else {
                    Ok(Self { file, lines })
                }
            }
            Err(e) => Err(Error::new(
                format!("io error: {}", e.to_string()),
                Default::default(),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub line: u16,
    pub offset: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Span {
    pub line: u16,
    pub begin: u8,
    pub length: u8,
}

impl Span {
    fn new(line: u16, begin: u8, length: u8) -> Self {
        Self {
            line,
            begin,
            length,
        }
    }
    pub fn new_p(position: Position, length: u8) -> Self {
        Self {
            line: position.line,
            begin: position.offset,
            length,
        }
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.line != other.line {
            self.line.partial_cmp(&other.line)
        } else if self.line != other.line {
            self.begin.partial_cmp(&other.begin)
        } else {
            self.length.partial_cmp(&other.length)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    // To be done: file.
    // To be done: kind.
    pub message: String,
    pub span: Span,
}

impl Error {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

pub struct ErrorC<'a> {
    pub error: Error,
    pub context: &'a Context,
}
