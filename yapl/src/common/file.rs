use std::path::PathBuf;

#[allow(unused)]
pub struct Context {
    pub file: PathBuf,
    pub lines: Vec<String>,
}

impl Context {
    #[allow(unused)]
    fn new_read(file: PathBuf) -> Result<Self, Error> {
        match std::fs::read_to_string(&file) {
            Ok(s) => {
                let lines: Vec<_> = s.lines().map(|l| l.to_string()).collect();
                if lines.len() > 10000 {
                    Err(Error::new("too many lines".to_string(), Default::default()))
                } else if let Some(long) = lines.iter().position(|l| l.len() > 200) {
                    Err(Error::new(
                        "too long line".to_string(),
                        Span::new(
                            Position::new(long as u16, 0),
                            Position::new(long as u16, lines[long].len() as u8),
                        ),
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub line: u16,
    pub offset: u8,
}

impl Position {
    pub fn new(line: u16, offset: u8) -> Self {
        Self { line, offset }
    }

    pub fn mov(&mut self, rhs: u8) {
        self.offset += rhs
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.line != other.line {
            self.line.partial_cmp(&other.line)
        } else {
            self.offset.partial_cmp(&other.offset)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Span {
    pub begin: Position,
    pub end: Position,
    pub _private: (),
}

impl Span {
    pub fn new(begin: Position, end: Position) -> Self {
        assert!(begin <= end);
        Self {
            begin,
            end,
            _private: (),
        }
    }
    pub fn new_p(position: Position, length: u8) -> Self {
        Self::new(
            position,
            Position::new(position.line, position.offset + length),
        )
    }

    pub fn new_s(line: u16, begin: u8, end: u8) -> Self {
        Self::new(Position::new(line, begin), Position::new(line, end))
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[allow(unused)]
pub struct ErrorC<'a> {
    pub error: Error,
    pub context: &'a Context,
}
