use flexar::{token_node::Node, Flext, token_node::TokenToString};
use crate::lang::{Token, errors::SyntaxError};
use super::Call;

#[derive(Debug, Clone)]
pub enum BinOp {
    Add(Box<Node<BinOp>>, Box<Node<BinOp>>),
    Sub(Box<Node<BinOp>>, Box<Node<BinOp>>),
    Mul(Node<Call>, Box<Node<BinOp>>),
    Div(Node<Call>, Box<Node<BinOp>>),
    EE(Box<Node<BinOp>>, Box<Node<BinOp>>),
    NE(Box<Node<BinOp>>, Box<Node<BinOp>>),
    Atom(Node<Call>),
}

flexar::parser! {
    [[BinOp] parxt: Token]
    parse {
        [left: BinOp::add_sub] => {
            (Token::EE), [right: BinOp::parse] => (EE(Box::new(left), Box::new(right)));
            (Token::NE), [right: BinOp::parse] => (NE(Box::new(left), Box::new(right)));
        } (else Ok(left.node))
    } else Err((SY004, parxt.position()) parxt.current_token());

    add_sub {
        [left: BinOp::mul_div] => {
            (Token::Plus), [right: BinOp::add_sub] => (Add(Box::new(left), Box::new(right)));
            (Token::Minus), [right: BinOp::add_sub] => (Sub(Box::new(left), Box::new(right)));
        } (else Ok(left.node))
    } else Err((SY004, parxt.position()) parxt.current_token());

    mul_div {
        [left: Call::parse] => {
            (Token::Mul), [right: BinOp::parse] => (Mul(left, Box::new(right)));
            (Token::Div), [right: BinOp::parse] => (Div(left, Box::new(right)));
        } (else Ok(Self::Atom(left)))
    } else Err((SY004, parxt.position()) parxt.current_token());
}