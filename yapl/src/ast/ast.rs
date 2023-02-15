// To be done: there will be structure and access actions on objects in this file.

use crate::common::location::Span;
use crate::common::symbol::Symbol;

use super::object::Object;

pub type Id = crate::common::space::Id;
pub type Space = crate::common::space::Space<Object>;

pub struct Project {
    space: Space,
    // files: HashMap<PathBuf, File>,
    root: Node,
}

pub struct Node {
    ty: NodeT,
    span: Span,
    symbol: Symbol,
    block: Vec<Node>,
    extension: Vec<Node>, // Only for NodeT::Line for now.
}

pub enum NodeT {
    Root,
    File,
    Line,
    Tuple,
    Set,
    Range,
}

// To be done: this should be able to modify ast.
pub struct Access<'project> {
    ast: &'project Project,
}
