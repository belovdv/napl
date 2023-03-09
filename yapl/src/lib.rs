#![feature(once_cell)]
mod common;

mod ast;
mod dfg;
mod parser;

mod glue;

/// `location::Context as File` -> Result<parser::Ast>.
pub use parser::parse;

/// parser::Ast -> ast::Ast.
pub use glue::parser2ast::parser2ast;

pub use common::error::Result;
pub use common::location::File;

pub use ast::Project;
