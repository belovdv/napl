// Builtin function implementations and initializing.

use crate::ast::context::Context;
use crate::common::{space::Space, symbol::Symbol};

use super::Object;

pub fn initialize(space: &mut Space<Object>, context: &mut Context) {}

//
// pub use action::Action;
// pub mod action {
//     pub enum Action {
//         User,
//         Arithmetic(Arithmetic),
//     }

//     pub enum Arithmetic {
//         Sum,   // Take many terms.
//         Mul,   // Take many terms.
//         Minus, // Take one term.
//         Pow,   // Take two terms.
//     }
// }
