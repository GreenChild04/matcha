use flexar::{Flext, token_node::{TokenToString, Node}};
use crate::lang::{Token, errors::SyntaxError, nodes::Atom};

#[derive(Debug, Clone)]
pub enum Call {
    Atom(Node<Atom>),
    Call(Node<Atom>, Box<Node<Call>>),
}

flexar::parser! {
    [[Call] parxt: Token]
    parse {
        [func: Atom::parse] => {
            [arg: Call::parse] => (Call(func, Box::new(arg)));
        } (else Ok(Call::Atom(func)))
    } else Err((SY007, parxt.position()) parxt.current_token());
}