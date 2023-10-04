use flexar::{token_node::Node, Flext, token_node::TokenToString};
use crate::lang::{Token, errors::SyntaxError, nodes::Expr};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Node<Expr>),
    Return(Node<Expr>),
}

flexar::parser! {
    [[Stmt] parxt: Token]
    parse {
        (Token::Return) => {
            [expr: Expr::parse] => {
                (Token::Semi) => (Return(expr));
            } (else Err((SY005, parxt.position()) parxt.current_token()))
        };

        [expr: Expr::parse] => {
            (Token::Semi) => (Expr(expr));
        } (else Ok(Stmt::Return(expr)))
    } else Err((SY006, parxt.position()) parxt.current_token());
}