use crate::common::error::Result;
use crate::common::location::Span;
use crate::common::symbol::Symbol;
use crate::common::Id as Obj;

use super::context::ContextPart;

// pub type Id = crate::common::space::Id;
// pub type Space = crate::common::space::Space<Value>;

pub struct Project {
    // space: Space,
    // files: HashMap<PathBuf, File>,
    roots: Vec<Line>,
}

#[derive(Debug, Clone, derive_new::new)]
pub struct Line {
    line: NodeS,
    extension: Vec<Line>,
    block: Vec<Line>,
    // #[new(default)]
    // context: Context<Object>,
    span: Span,
}

pub enum Value {
    Object(Obj),
    Action(Action),
}

impl PartialEq for Value {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, derive_new::new)]
pub struct NodeS {
    node: Node,
    span: Span,
}

#[derive(Debug, Clone, Default)]
pub enum Action {
    #[default]
    None,
}

#[derive(Debug, Clone)]
pub enum Node {
    Phrase(Vec<NodeS>),
    Bracket(Bracket, Vec<NodeS>),
    Chain(Vec<Symbol>),
    LitStr(String),
    LitInt(i64),
}

#[derive(Debug, Clone, Copy)]
pub enum Bracket {
    Round,
    Square,
    Curly,
}

// To be done: this should be primary way to access ast.
// To be done: this should be able to modify ast.
pub struct Access<'project> {
    ast: &'project Project,
}

impl Project {
    pub fn new(roots: Vec<Line>) -> Self {
        // let space = Space::default();
        Self { /* space,*/ roots, }
    }

    pub fn interpret(self) -> Result<Vec<Obj>> {
        let mut context = Default::default();
        // let mut space = self.space;
        let roots: Result<Vec<_>> = self
            .roots
            .into_iter()
            .map(|r| r.act(/* &mut space, */ &mut context))
            .collect();
        roots.map(|v| v.into_iter().flatten().collect())
    }
}

impl Line {
    fn act(
        self,
        /* space: &mut Space, */
        context: &mut ContextPart,
    ) -> Result<Option<Obj>> {
        let line = self.line;
        let mut extension = self.extension;
        let mut block = self.block;
        // let span = self.span;
        line.process_sub_lines(&mut extension, &mut block, /* space, */ context);

        Ok(None)
    }
}

impl NodeS {
    fn process_sub_lines(
        &self,
        _extension: &mut Vec<Line>,
        _block: &mut Vec<Line>,
        // _space: &mut Space,
        _context: &mut ContextPart,
    ) {
    }
}

// To be done: macro.
impl NodeS {
    pub fn new_p(phrase: Vec<NodeS>, span: Span) -> Self {
        let node = Node::Phrase(phrase);
        Self { node, span }
    }
    pub fn new_c(chain: Vec<Symbol>, span: Span) -> Self {
        let node = Node::Chain(chain);
        Self { node, span }
    }
    pub fn new_br(inner: Vec<NodeS>, span: Span) -> Self {
        let node = Node::Bracket(Bracket::Round, inner);
        Self { node, span }
    }
    pub fn new_bs(inner: Vec<NodeS>, span: Span) -> Self {
        let node = Node::Bracket(Bracket::Square, inner);
        Self { node, span }
    }
    pub fn new_bc(inner: Vec<NodeS>, span: Span) -> Self {
        let node = Node::Bracket(Bracket::Curly, inner);
        Self { node, span }
    }
    pub fn new_ls(lit_str: String, span: Span) -> Self {
        let node = Node::LitStr(lit_str);
        Self { node, span }
    }
    pub fn new_li(lit_int: i64, span: Span) -> Self {
        let node = Node::LitInt(lit_int);
        Self { node, span }
    }
}
