/// Basic grammatical rules.
/// This is, if it will be possible, will be simplified.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SymbolType {
    /// End of stream.
    EOS,
    /// ".".
    /// Separates `name chain`.
    /// Also reserved for compiler and used for comments.
    Dot,
    /// ",".
    /// Separates parts of bracket substring.
    Comma,
    /// "\"".
    /// Defines `string`.
    Quote,
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
    Whitespace(u8),
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
            c if c.is_alphabetic() || c == '_' => Self::Letter(c),
            c if c.is_ascii_digit() => Self::Digit(c),
            c if "<>+-*/=&|^!'#".contains(c) => Self::Special(c),
            '(' => Self::Bracket(BracketType::Round, true),
            '[' => Self::Bracket(BracketType::Square, true),
            '{' => Self::Bracket(BracketType::Curly, true),
            ')' => Self::Bracket(BracketType::Round, false),
            ']' => Self::Bracket(BracketType::Square, false),
            '}' => Self::Bracket(BracketType::Curly, false),
            ' ' => Self::Whitespace(1),
            '\t' => Self::Whitespace(TAB_TO_SPACES as u8),
            '\n' => Self::NewLine,
            _ => Self::Other,
        }
    }
}

const TAB_TO_SPACES: usize = 2;
pub fn offset(offset_in_spaces: usize) -> Option<usize> {
    match offset_in_spaces {
        o if o % TAB_TO_SPACES == 0 => Some(o / TAB_TO_SPACES),
        _ => None,
    }
}
