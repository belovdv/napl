use crate::common::location::Span;
use crate::common::symbol::Symbol;

use super::context::ContextPart;
use super::{builtin, project};

#[derive(Debug, Clone, derive_new::new)]
pub struct Line {
    line: NodeS,
    extension: Vec<Line>,
    block: Vec<Line>,
    span: Span,
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

// To be done: these placeholders should be more generic.

impl Line {
    // To be done: this is placeholder.
    pub fn act(self, _context: &mut ContextPart) -> project::Function {
        let line = self.line;
        // let mut extension = self.extension;
        let block = self.block;
        // let span = self.span;

        let Node::Phrase(phrase) = &line.node else {
            panic!()
        };

        match &phrase[0].node {
            Node::Chain(c) if c[0] == "." && c[1] == "fn" => {}
            _ => panic!(),
        }

        let name = match &phrase[1].node {
            Node::Chain(v) if v.len() == 1 => v[0],
            _ => panic!(),
        };

        let args = match &phrase[2].node {
            Node::Bracket(Bracket::Round, args) => args,
            _ => panic!(),
        };
        let args = args
            .iter()
            .map(|s| match &s.node {
                Node::Phrase(p) if p.len() == 3 => match &p[0].node {
                    Node::Chain(c) if c.len() == 1 => c[0],
                    _ => panic!(),
                },
                _ => panic!(),
            })
            .collect();

        let stmts = block.into_iter().map(|n| n.subline()).collect();

        project::Function { name, args, stmts }
    }

    fn subline(self) -> project::Statement {
        let vsp = self.line.node.process(&self.block);
        dbg!(&vsp);
        vsp
    }
}

impl Node {
    fn process(self, block: &Vec<Line>) -> project::Statement {
        match self {
            Node::Phrase(p) => Self::process_phrase(&mut p.into_iter(), &block),
            Node::Bracket(Bracket::Curly, _) => todo!(),
            Node::Chain(c) => {
                let base = project::Statement::Symbol(c[0]);
                match c.len() {
                    1 => base,
                    _ => project::Statement::Inner {
                        from: Box::new(base),
                        with: c[1..].iter().map(|&c| c).collect(),
                    },
                }
            }
            Node::LitStr(s) => project::Statement::ConstantS(s),
            Node::LitInt(i) => project::Statement::ConstantI(i),
            _ => panic!(),
        }
    }

    fn process_phrase(
        phrase: &mut dyn Iterator<Item = NodeS>,
        block: &Vec<Line>,
    ) -> project::Statement {
        match phrase.next().unwrap().node.process(block) {
            project::Statement::Symbol(s) => match builtin::operator(s) {
                Some(num) => project::Statement::Mapped {
                    func: s,
                    args: (0..num)
                        .map(|_| Self::process_phrase(phrase, block))
                        .collect(),
                },
                None => project::Statement::Symbol(s),
            },
            // project::Statement::Set { ty, args, stmts } => todo!(),
            s => s,
        }
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
