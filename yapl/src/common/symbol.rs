use std::{
    fmt::Debug,
    sync::{LazyLock, Mutex},
};

use super::space::{Id, Space};

static mut SYMBOLS: LazyLock<Mutex<Space<String>>> = LazyLock::new(|| Default::default());

#[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Symbol(Id);

impl Symbol {
    // pub fn as_str(&self) -> &str {
    //     unsafe { SYMBOLS.lock().unwrap().get(self.0) }
    // }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Self(unsafe { SYMBOLS.lock().unwrap().insert_unique(value) })
    }
}

impl From<&'static str> for Symbol {
    fn from(value: &'static str) -> Self {
        value.to_string().into()
    }
}

impl ToString for Symbol {
    // To be done: replace by `as_str`.
    fn to_string(&self) -> String {
        unsafe { SYMBOLS.lock().unwrap().get(self.0).to_string() }
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Symbol({}: \"{}\")",
            self.0.as_u32(),
            self.to_string()
        ))
    }
}

impl PartialEq<&str> for Symbol {
    fn eq(&self, other: &&str) -> bool {
        // To be done: replace by `as_str`.
        self.to_string() == other.to_string()
    }
}

impl PartialEq<str> for Symbol {
    fn eq(&self, other: &str) -> bool {
        // To be done: replace by `as_str`.
        self.to_string() == other.to_string()
    }
}
