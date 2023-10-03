use std::fmt::Display;
use flexar::{parser, Flext, token_node::TokenToString};
use crate::lang::{Token, errors::SyntaxError};

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Int(u128),
    Float(f64),
}

parser! {
    [[Literal] parxt: Token]
    parse {
        (Token::Str(x)) => (Str(x.clone()));
        (Token::Int(x)) => (Int(*x));
        (Token::Float(x)) => (Float(*x));
    } else Err((SY001, parxt.position()) parxt.current_token());
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(x) => write!(f, "{x:?}"),
            Self::Int(x) => write!(f, "{x:?}"),
            Self::Float(x) => write!(f, "{x:?}"),
        }
    }
}