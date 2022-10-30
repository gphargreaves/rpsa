#![allow(unused)]

mod lexer;
use lexer::token::*;
use lexer::*;

mod file_watcher;
use file_watcher::*;

fn main() {
    let mut fw: FileWatcher = FileWatcher::create("./extra/");
    fw.watch();

    /*

    let input: String = std::fs::read_to_string("/Users/greg/rpsa/extra/test.php").unwrap();
    let mut lex: Lexer = Lexer::new(input.as_str());

    loop {
        match lex.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(token) => println!("{0:?}", token),
            Err(err) => println!("{0:?}", err)
        }
    }
     */
}
