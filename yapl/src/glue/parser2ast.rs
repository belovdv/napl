// This is necessary to change modules (almost) independently.

use crate::ast::ast;
use crate::common::location::Span;
use crate::parser::ast as parser_ast;

use crate::common::error::Result;

// To be done: `Result` is here to handle possible incompatibility. Is it usefull?
pub fn parser2ast(parser: &parser_ast::File) -> Result<Vec<ast::Line>> {
    parser.roots.iter().map(p2a_line).collect()
}

fn p2a_line(line: &parser_ast::Line) -> Result<ast::Line> {
    let block: Vec<_> = line.block.iter().map(p2a_line).collect::<Result<_>>()?;
    let extension: Vec<_> = line.extension.iter().map(p2a_line).collect::<Result<_>>()?;
    let sent = p2a_sent(&line.sent);
    let span_tree = line.span
        + match (block.len(), extension.len()) {
            (_, e) if e > 0 => extension[e - 1].span,
            (b, 0) if b > 0 => block[b - 1].span,
            (0, 0) => sent.span,
            _ => unreachable!(),
        };
    Ok(ast::Line::new(sent, extension, block, span_tree))
}

fn p2a_sent(sent: &parser_ast::Sent) -> ast::Sent {
    ast::Sent::new(sent.chains.iter().map(p2a_chain).collect(), sent.span)
}

fn p2a_chain(chain: &parser_ast::Chain) -> ast::Chain {
    ast::Chain::new(chain.chain.iter().map(p2a_expr).collect(), chain.span)
}

fn p2a_expr(expr: &parser_ast::Expr) -> (ast::Expr, Span) {
    let ast_expr = match &expr.expr {
        parser_ast::ExprT::Dot => ast::Expr::Inner,
        parser_ast::ExprT::Symbol(s) => ast::Expr::Symbol(*s),
        parser_ast::ExprT::Bracket { ty, inner } => ast::Expr::Bracket {
            ty: *ty,
            inner: inner.iter().map(p2a_sent).collect(),
        },
        parser_ast::ExprT::LitStr(s) => ast::Expr::LitStr(s.clone()),
        parser_ast::ExprT::LitInt(i) => ast::Expr::LitInt(*i),
    };
    (ast_expr, expr.span)
}
