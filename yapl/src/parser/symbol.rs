/// Basic grammatical rules.
/// This is, if it will be possible, will be simplified.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SymbolType {
    /// End of stream.
    EOS,
    /// ".".
    /// Separates `name chain`.
    Dot,
    /// ",".
    /// Separates parts of bracket substring.
    Comma,
    /// "\"".
    /// Defines `string`.
    Quote,
    /// "#".
    /// Reserved for compiler.
    /// Used for comments.
    Inner,
    /// is_alphabetic or "_".
    /// Part of `name` or `literal`.
    Letter(char),
    /// is_ascii_digit.
    /// Part of `name` or `literal`.
    Digit(char),
    /// "()[]{}".
    /// Described alone as it'c basis of language.
    /// Type, is_open.
    Bracket(BracketType, bool),
    /// "<> +-*/= &|^! '".
    /// Unites in `special`.
    Special(char),
    /// " \t".
    /// Defines indent and separates tokens.
    Whitespace(char),
    /// "\n".
    /// Separates lines.
    NewLine,
    /// Any other unicode.
    /// Forbidden to be used (except `string`).
    Other,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BracketType {
    Round,
    Square,
    Curly,
}

impl From<Option<char>> for SymbolType {
    fn from(symbol: Option<char>) -> Self {
        symbol.map(|c| c.into()).unwrap_or(Self::EOS)
    }
}

impl From<char> for SymbolType {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Dot,
            ',' => Self::Comma,
            '"' => Self::Quote,
            '#' => Self::Inner,
            c if c.is_alphabetic() || c == '_' => Self::Letter(c),
            c if c.is_ascii_digit() => Self::Digit(c),
            '(' => Self::Bracket(BracketType::Round, true),
            '[' => Self::Bracket(BracketType::Square, true),
            '{' => Self::Bracket(BracketType::Curly, true),
            ')' => Self::Bracket(BracketType::Round, false),
            ']' => Self::Bracket(BracketType::Square, false),
            '}' => Self::Bracket(BracketType::Curly, false),
            c if "<>+-*/=&|^!'".contains(c) => Self::Special(c),
            ' ' | '\t' => Self::Whitespace(c),
            '\n' => Self::NewLine,
            _ => Self::Other,
        }
    }
}

pub const TAB_TO_SPACES: u8 = 2;
pub fn offset(c: Option<char>) -> Option<u8> {
    match c {
        Some(' ') => Some(1),
        Some('\t') => Some(TAB_TO_SPACES),
        _ => None,
    }
}
