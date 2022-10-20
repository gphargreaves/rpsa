mod tokenizer;
use tokenizer::Tokenizer;

fn main() {
    let mut tk: Tokenizer = Tokenizer::from_filepath("/Users/greg/rpsa/extra/test.php");
    while tk.has_more_tokens() {
        let t  =  tk.get_next_token();
        println!("<{}:{}> Token Type: {} Token: {}", t.get_line(), t.get_col(), t.get_token_type(), t.get_value());
    }
}