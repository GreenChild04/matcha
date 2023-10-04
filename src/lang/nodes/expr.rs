use flexar::{token_node::Node, Flext, token_node::TokenToString};
use crate::lang::{Token, errors::SyntaxError};

use super::BinOp;

#[derive(Debug, Clone)]
pub enum Expr {
    BinOp(Box<Node<BinOp>>),
}

flexar::parser! {
    [[Expr] parxt: Token]
    parse {
        [binop: BinOp::parse] => (BinOp(Box::new(binop)));
    } else Err((SY002, parxt.position()) parxt.current_token());
}