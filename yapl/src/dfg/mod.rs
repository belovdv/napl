mod builtin;
mod function;
mod set;
mod r#trait;
mod r#type;

use crate::common::space::Id as Obj;

pub use function::Function;
pub use r#trait::Trait;
pub use r#type::{Instance, Type};
pub use set::Set;

pub enum Object {
    Type(Type),
    Instance(Instance),
    Function(Box<dyn Function>),
    Produced { func: Obj, input: Obj },
}
