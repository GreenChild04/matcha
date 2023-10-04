use std::fs;
use flexar::{lext::Lext, parxt::Parxt};
use matcha::lang::{lexer::Token, nodes::Stmt};

fn main() {
    let tokens = Token::tokenize(Lext::new("example.mt".into(), &fs::read_to_string("example.mt").unwrap()));

    println!("Tokens: {:?}", 
        tokens
            .iter()
            .map(|x| &x.token_type)
            .collect::<Box<_>>()
    );

    let nodes = Stmt::parse(&mut Parxt::new(&tokens));
    match nodes {
        Ok(x) => println!("\nNodes: {:?}", x),
        Err((_, x)) => x.throw(),
    }
}
