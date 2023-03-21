#![feature(once_cell)]

mod common;
mod glue;

// To be done: remove `pub`.
pub mod ast;

// To be done: remove `unused`.
#[allow(unused)]
mod dfg;
mod parser;

/// `location::Context as File` -> Result<parser::Ast>.
pub use parser::parse;

/// parser::Ast -> ast::Ast.
pub use glue::parser2ast::parser2ast;
// pub use glue::ast22dfg::ast22dfg;

pub use common::error::Result;
pub use common::location::File;
