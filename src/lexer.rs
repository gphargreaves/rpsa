pub mod token;
use token::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {

    #[error("NonNumericLiteralInvalidChar: Invalid character in numeric literal {symbol:?} at line: {line:?} pos: {col:?}")]
    NonNumericLiteralInvalidChar {
        symbol: String,
        line: usize,
        col: usize
    },

    #[error("UnknownSymbol: No token rule matches symbol ->{symbol:?}<- at line: {line:?} pos: {col:?}")]
    UnknownSymbol {
        symbol: String,
        line: usize,
        col: usize
    },

    #[error("TokenDepthUnbalanced: Depth for symbol  {symbol:?} is 0 at line: {line:?} pos: {col:?}")]
    TokenDepthUnbalanced {
        symbol: char,
        line: usize,
        col: usize
    }
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    depth_state: std::collections::HashMap<char, DepthStateType>,
    cursor: usize,
    line: usize,
    col: usize
}

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str) -> Lexer<'a>{
        return Lexer {
            chars: chars.chars().peekable(),
            cursor: 0,
            line: 1,
            col: 1,
            depth_state: std::collections::HashMap::new()
        };
    }

    fn map_depth_chars(c: &char) -> char{
        match c {
            ')' => '(',
            '}' => '{',
            ']' => '[',
            _ => panic!("Trying to map an incorrect depth tracked char")
        }
    }

    fn push_depth_map(&mut self, c: char) -> DepthStateType{
        if let Some(v) = self.depth_state.get_mut(&c){
            *v += 1;
            *v - 1
        } else {
            self.depth_state.insert(c, 1);
            0
        }
    }

    fn pop_depth_map(&mut self, c: char) -> Result<DepthStateType, LexerError>{
        if let Some(v) = self.depth_state.get_mut(&Lexer::map_depth_chars(&c)){
            if *v >= 1 {
                *v -= 1;
                return Ok(*v);
            }else{
                return Err(LexerError::TokenDepthUnbalanced { symbol:c, line: self.line, col: self.col })
            }
        } else {
            return Err(LexerError::TokenDepthUnbalanced { symbol:c, line: self.line, col: self.col })
        }
    }

    fn is_punctuation(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' | '[' | '{' => Ok(TokenType::Punctuation { raw:c, kind: PunctuationKind::Open(self.push_depth_map(c)) }),
            ')' | ']' | '}' => Ok(TokenType::Punctuation { raw:c, kind: PunctuationKind::Close(self.pop_depth_map(c)?) }),
            '0'..='9' => self.parse_numeric(c),
            _ => Err(LexerError::UnknownSymbol { symbol: c.to_string(), line: self.line, col: self.col })
        }
    }

    fn parse_numeric(&mut self, start:char) -> Result<TokenType, LexerError> {
        let mut seen_dot = false;
        let mut seen_exp = false;
        let mut num = start.to_string();

        if start == '.' {
            seen_dot = true;
        }

        loop {
            match self.chars.peek() {
                Some(c) if *c == '.' && !seen_dot =>{
                    num.push(*c);
                    self.consume_char();
                    seen_dot = true;
                },
                Some(c) if (*c == 'e' || *c == 'E') && !seen_exp => {
                    num.push(*c);
                    self.consume_char();
                    seen_exp = true;

                    match self.chars.peek() {
                        Some(c) if *c == '-' || *c == '+' => {
                            num.push(*c);
                            self.consume_char();
                        },
                        _ => {}
                    }

                    match self.chars.peek() {
                        None => break Err(LexerError::NonNumericLiteralInvalidChar { symbol: num, line: self.line, col: self.col }),
                        Some(c) if !c.is_digit(10) => break Err(LexerError::NonNumericLiteralInvalidChar { symbol: num, line: self.line, col: self.col }),
                        _ => {}
                    }
                },
                Some(c) if c.is_digit(10) => {
                    num.push(*c);
                    self.consume_char();
                },
                _ => break Ok(TokenType::Numeric(num))
            }
        }

    }

    fn consume_char(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.col += 1;
                self.cursor += 1;
                if c == '\n' {
                    self.line += 1;
                    self.col = 1;
                }
                Some(c)
            },
            None => None
        }
    }

    pub fn skip_whitespace(&mut self){
        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace(){
                break;
            }
            self.consume_char();
        }
    }

    pub fn next_token(&mut self) -> Result<TokenType, LexerError>{
        self.skip_whitespace();

        if let Some(c) = self.consume_char() {
            self.is_punctuation(c)
        } else {
            Ok(TokenType::EOF)
        }
    }

}