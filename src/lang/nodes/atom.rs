use flexar::{Flext, token_node::{TokenToString, Node}};
use crate::lang::{Token, errors::SyntaxError};
use super::Literal;

#[derive(Debug, Clone)]
pub enum Atom {
    Literal(Node<Literal>),
}

flexar::parser! {
    [[Atom] parxt: Token]
    parse {
        [literal: Literal::parse] => (Literal(literal));
    } else Err((SY003, parxt.position()) parxt.current_token());
}