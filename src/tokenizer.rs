mod token;
use token::Token;
use regex::Regex;
use std::fs;

const T_WHITESPACE: &str = "T_WHITESPACE";

//Structural
const T_PHP_OPEN_TAG: &str = "T_PHP_OPEN_TAG";
const T_PHP_CLOSE_TAG: &str = "T_PHP_CLOSE_TAG";
const T_SEMICOLON: &str = "T_SEMICOLON";
const T_COLON: &str = "T_COLON";
const T_FULL_STOP: &str = "T_FULL_STOP";
const T_OPEN_CURLY: &str = "T_OPEN_CURLY";
const T_CLOSE_CURLY: &str = "T_CLOSE_CURLY";
const T_OPEN_PARANTHESES: &str = "T_OPEN_PARANTHESES";
const T_CLOSE_PARANTHESES: &str = "T_CLOSE_PARANTHESES";
const T_EQUALS: &str = "T_EQUALS";
const T_IS_IDENTICAL: &str = "T_IS_IDENTICAL";

//Reserved Words
const T_CLASS: &str = "T_CLASS";
const T_STATIC: &str = "T_STATIC";
const T_PUBLIC: &str = "T_PUBLIC";
const T_PRIVATE: &str = "T_PRIVATE";
const T_PROTECTED: &str = "T_PROTECTED";
const T_FUNCTION: &str = "T_FUNCTION";
const T_IF: &str = "T_IF";
const T_ECHO: &str = "T_ECHO";

//Other
const T_VARIABLE: &str = "T_VARIABLE";
const T_STRING: &str = "T_STRING";

//Literals
const T_LSTRING: &str = "T_LSTRING";
const T_LNUMBER: &str = "T_LNUMBER";

const TOKEN_RULES: [(&str, &str); 25] = [
    //Whitespace
    (T_WHITESPACE, r"^\s+"),
    //PHP Structural
    (T_PHP_OPEN_TAG, r"^<\?php\s"),
    (T_PHP_CLOSE_TAG, r"^\?>"),
    (T_OPEN_CURLY, r"^\{"),
    (T_CLOSE_CURLY, r"^\}"),
    (T_OPEN_PARANTHESES, r"^\("),
    (T_CLOSE_PARANTHESES, r"^\)"),
    (T_SEMICOLON, r"^;"),
    (T_COLON, r"^:"),
    (T_FULL_STOP, r"^\."),
    (T_IS_IDENTICAL, r"^==="),
    (T_EQUALS, r"^="),
    //PHP Reserved
    (T_CLASS, r"^class"),
    (T_STATIC, r"^static"),
    (T_PUBLIC, r"^public"),
    (T_PRIVATE, r"^private"),
    (T_PROTECTED, r"^protected"),
    (T_FUNCTION, r"^function"),
    (T_IF, r"^if"),
    (T_ECHO, r"^echo"),

    //Other
    (T_VARIABLE, r"^\$[[:alpha:]_]+"),
    (T_STRING, r"^[[:alpha:]]+"),
    //Literals
    (T_LSTRING, r#"^"[^"]*""#),
    (T_LSTRING, r#"^'[^']*'"#),
    (T_LNUMBER, r"^\d+"),
];
pub struct Tokenizer {
    code: String,
    cursor: usize,
    line: usize,
    col: usize
}

#[allow(dead_code)]
impl Tokenizer {
    pub fn new(code: &str) -> Tokenizer{
        return Tokenizer {code: String::from(code), cursor: 0, line: 1, col: 1};
    }

    pub fn from_filepath(filepath: &str) -> Tokenizer{
        let content: String = fs::read_to_string(filepath).expect("Should have been able to read;");
        return Tokenizer { code: content, cursor: 0, line: 1, col: 1};
    }

    pub fn has_more_tokens(&self) -> bool{
        return self.cursor < self.code.len()
    }

    pub fn get_next_token(&mut self) -> Token{
        let original: String = self.code.clone();
        let target: &str = &original.as_str()[self.cursor..];
        //println!("Scanned code: <#>{}<#>", target);

        for rule in TOKEN_RULES{
            let result: Option<Token> = self.match_token(target, rule);

            if result.is_none() {
                continue;
            }

            let token_result: Token = result.unwrap();

            let new_lines: usize =  token_result.get_value().matches("\n").count();
            self.line += new_lines;

            let re = Regex::new(r"\n.*$").unwrap();
            let matched = re.find(token_result.get_value());
            if matched.is_some() {
                let matched_str: &str = matched.unwrap().as_str();
                self.col = matched_str.len();
            }

            return token_result;

        }
       
        panic!("Unrecognised token found in code");
    }

    fn match_token(&mut self, target: &str, definition: (&str, &str)) -> Option<Token>{
        let re = Regex::new(definition.1).unwrap();
        let matched = re.find(target);
        if matched.is_some() {
            let matched_str: &str = matched.unwrap().as_str();
            self.cursor += matched_str.len();
            let result: Option<Token> =  Option::Some(Token::new(definition.0, matched_str, self.line, self.col));
            self.col += matched_str.len();
            return result;
        }
        return Option::None;
    }
}