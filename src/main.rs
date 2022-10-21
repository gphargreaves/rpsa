mod lexer;
use lexer::Lexer;

fn main() {
    let mut tk: Lexer = Lexer::new("/Users/greg/rpsa/data/php-tokens.json");
    tk.init_from_filepath("/Users/greg/rpsa/extra/test.php");

    while tk.has_more_tokens() {
        let t  =  tk.get_next_token().ok().unwrap();
        println!("<{}:{}> Token Type: {} Token: {}", t.get_line(), t.get_col(), t.get_token_type(), t.get_value());
    }
}