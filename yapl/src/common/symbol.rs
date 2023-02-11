#[derive(Debug, Clone, PartialEq)]
pub struct Symbol(String); // To be done: pool for strings with u32 keys.

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Symbol(value)
    }
}

impl From<&'static str> for Symbol {
    fn from(value: &'static str) -> Self {
        Symbol(value.to_string())
    }
}
