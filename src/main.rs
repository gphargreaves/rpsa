mod lexer;
use lexer::*;
use lexer::token::*;

fn main() {
    let mut lex: Lexer = Lexer::new("(0.2)");

    loop {
        match lex.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(token) => println!("{0:?}", token),
            Err(err) => println!("{0:?}", err)
        }
    }
}