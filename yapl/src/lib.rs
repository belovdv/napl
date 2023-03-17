#![feature(once_cell)]

mod common;
mod glue;

// To be done: remove this `pub`.
pub mod ast;

mod dfg;
mod parser;

/// `location::Context as File` -> Result<parser::Ast>.
pub use parser::parse;

/// parser::Ast -> ast::Ast.
pub use glue::parser2ast::parser2ast;
// pub use glue::ast22dfg::ast22dfg;

pub use common::error::Result;
pub use common::location::File;
