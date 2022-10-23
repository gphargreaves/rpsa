mod lexer;
use lexer::*;
use lexer::token::*;

fn main() {
    let input: String = std::fs::read_to_string("/Users/greg/rpsa/extra/test.php").unwrap();
    let mut lex: Lexer = Lexer::new(input.as_str());

    loop {
        match lex.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(token) => println!("{0:?}", token),
            Err(err) => println!("{0:?}", err)
        }
    }
}