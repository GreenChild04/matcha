use std::fs;
use flexar::lext::Lext;
use matcha::lang::lexer::Token;

fn main() {
    println!("{:?}", 
        Token::tokenize(Lext::new("example.mt".into(), &fs::read_to_string("example.mt").unwrap()))
            .iter()
            .map(|x| &x.token_type)
            .collect::<Box<_>>()
    );
}
