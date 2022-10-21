mod token;
mod rules;
use token::Token;
use rules::Rules;
use regex::Regex;
use std::{fs};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {

    #[error("No token rule matches at line: {line:?} pos: {col:?}")]
    UnknownSymbol {
        line: usize,
        col: usize
    }
}

pub struct Lexer {
    rules: Rules,
    code: String,
    cursor: usize,
    line: usize,
    col: usize
}

#[allow(dead_code)]
impl Lexer {
    pub fn new(rules_filepath: &str) -> Lexer{
        let rules: Rules = Rules::from_filepath(rules_filepath);
        return Lexer {rules, code: String::new(), cursor: 0, line: 1, col: 1};
    }

    pub fn init_from_filepath(&mut self, filepath: &str){
        let content: String = fs::read_to_string(filepath).expect("Should have been able to read;");
        self.code = content;
    }

    pub fn has_more_tokens(&self) -> bool{
        return self.cursor < self.code.len()
    }

    pub fn get_next_token(&mut self) -> Result<Token, LexerError>{
        let original: String = self.code.clone();
        let target: &str = &original.as_str()[self.cursor..];
        //println!("Scanned code: <#>{}<#>", target);

        for rule in self.rules.get_rules(){
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
            return Ok(token_result);
        }
        return Err(LexerError::UnknownSymbol { line: self.line, col: self.col });
    }

    fn match_token(&mut self, target: &str, definition: (String, Regex)) -> Option<Token>{
        let re = definition.1;
        let matched = re.find(target);
        if matched.is_some() {
            let matched_str: &str = matched.unwrap().as_str();
            self.cursor += matched_str.len();
            let result: Option<Token> =  Option::Some(Token::new(definition.0.as_str(), matched_str, self.line, self.col));
            self.col += matched_str.len();
            return result;
        }
        return Option::None;
    }
}