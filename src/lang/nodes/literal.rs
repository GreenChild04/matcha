use flexar::{Flext, token_node::TokenToString};
use crate::lang::{Token, errors::SyntaxError};

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    StrTplt(Box<[String]>),
    Int(u128),
    Float(f64),
}

flexar::parser! {
    [[Literal] parxt: Token]
    parse {
        (Token::Str(x)) => (Str(x.clone()));
        (Token::StrTplt(x)) => (StrTplt(x.clone()));
        (Token::Int(x)) => (Int(*x));
        (Token::Float(x)) => (Float(*x));
    } else Err((SY001, parxt.position()) parxt.current_token());
}