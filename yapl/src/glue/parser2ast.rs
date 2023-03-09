use std::vec;

use crate::ast::ast;
use crate::common::symbol::Symbol;
use crate::parser::ast as parser_ast;

// To be done: don't stop on first error.
use crate::common::error::{raise_error_on, Result};
use crate::common::location::HasSpan;

use super::errors::ToBeDone;

pub fn parser2ast(parser: &parser_ast::File) -> Result<Vec<ast::Line>> {
    parser.roots().iter().map(p2a_line).collect()
}

fn p2a_line(line: &parser_ast::Line) -> Result<ast::Line> {
    let block: Result<Vec<_>> = line.block.iter().map(p2a_line).collect();
    let extension: Result<Vec<_>> = line.extension.iter().map(p2a_line).collect();
    let sent = p2a_sent(&line.sent);
    Ok(ast::Line::new(sent?, extension?, block?, line.span))
}

fn p2a_sent(sent: &parser_ast::Sent) -> Result<ast::NodeS> {
    let phrase: Result<Vec<_>> = sent.sent.iter().map(p2a_expr).collect();
    phrase.map(|p| ast::NodeS::new_p(p, sent.span))
}

// To be done: make code at least a little better...
fn p2a_expr(expr: &parser_ast::Expr) -> Result<ast::NodeS> {
    Ok(match &expr.expr {
        parser_ast::ExprT::Inner(i) => match &i.expr {
            parser_ast::ExprT::Chain(c) => ast::NodeS::new_c(p2a_dot_chain(c), expr.span),
            _ => raise_error_on!(ToBeDone, expr,),
        },
        parser_ast::ExprT::Chain(c) => ast::NodeS::new_c(c.to_vec(), expr.span),
        parser_ast::ExprT::Special(s) => ast::NodeS::new_c(vec![s.clone()], expr.span),
        parser_ast::ExprT::LitStr(s) => ast::NodeS::new_ls(s.clone(), expr.span),
        parser_ast::ExprT::LitInt(i) => ast::NodeS::new_li(*i, expr.span),
        parser_ast::ExprT::Bracket(bt, sentences) => {
            let sentences: Result<_> = sentences.iter().map(p2a_sent).collect();
            match bt {
                parser_ast::BracketType::Round => ast::NodeS::new_br(sentences?, expr.span),
                parser_ast::BracketType::Square => ast::NodeS::new_bs(sentences?, expr.span),
                parser_ast::BracketType::Curly => ast::NodeS::new_bc(sentences?, expr.span),
            }
        }
    })
}

fn p2a_dot_chain(chain: &Vec<Symbol>) -> Vec<Symbol> {
    vec![".".into()].into_iter().chain(chain.clone()).collect()
}
