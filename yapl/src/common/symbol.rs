use std::fmt::Debug;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Symbol(\"{}\")", self.0))
    }
}
