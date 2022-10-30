pub mod token;
pub mod keywords;

use token::*;
use keywords::parse_keyword;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {

    #[error("NonNumericLiteralInvalidChar: Invalid character in numeric literal {symbol:?} at line: {line:?} pos: {col:?}")]
    NonNumericLiteralInvalidChar {
        symbol: String,
        line: usize,
        col: usize
    },

    #[error("MissingExpectedSymbol: Expected {expected:?} but found {found:?} at line: {line:?} pos: {col:?}")]
    MissingExpectedSymbol {
        expected: String,
        found: TokenType,
        line: usize,
        col: usize
    },

    #[error("UnknownOperator: Found unkown operator {raw:?} at line: {line:?} pos: {col:?}")]
    UnknownOperator {
        raw: String,
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

macro_rules! try_consume {
    ($self: tt, $($inner:tt),*) => {
        if let Some(c) = $self.chars.peek() {
            if try_consume!(impl c, $($inner),*) {
                let tmp = *c;
                $self.consume_char();
                Some(tmp)
            } else {
                None
            }
        } else {
            None
        }
    };

    (impl , ) => (false);
    (impl $c:tt, $item:tt) => (*$c == $item);
    (impl $c:tt, $item:tt, $($rest:tt),+) => (try_consume!(impl $c, $item) || try_consume!(impl $c, $($rest),*));
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

    fn parse_digits(&mut self, radix: u32, allow_empty: bool) -> Result<String, LexerError> {
        let mut num = String::new();
        loop {
            match self.chars.peek() {
                None => {
                    break if allow_empty || num.len() > 0 {
                        Ok(num)
                    } else {
                        Err(LexerError::MissingExpectedSymbol { 
                            expected: "<number>".to_string(), 
                            found: TokenType::EOF, 
                            line: self.line, 
                            col: self.col 
                        })
                    }
                },
                Some(c) if c.is_digit(radix) || (*c == '_' && num.len() > 0) => num.push(self.consume_char().unwrap()),
                Some(c) if !c.is_ascii_alphabetic() && *c != '_' => break Ok(num),
                Some(_c) => {
                    break Err(LexerError::NonNumericLiteralInvalidChar { 
                        symbol: num, 
                        line: self.line, 
                        col: self.col 
                    })
                }
            }
        }
    }

    fn parse_numeric(&mut self, start:char) -> Result<TokenType, LexerError> {
        let mut num = start.to_string();
        let mut hint: NumericHint = NumericHint::Integer;

        let radix: u32 = 10;

        if start == '.' {
            num += &self.parse_digits(radix, false)?;
            hint = NumericHint::Float;
        } else if start.is_digit(radix) {
            num += &self.parse_digits(radix, false)?;

            if let Some(c) = try_consume!(self, '.') {
                num.push(c);
                num += &self.parse_digits(radix, false)?;
                hint = NumericHint::Float;
            }
        } else {
            return Err(LexerError::NonNumericLiteralInvalidChar { 
                symbol: num, 
                line: self.line, 
                col: self.col 
            });
        }

        if let Some(c) = try_consume!(self, 'e', 'E') {
            num.push(c);

            if let Some(c) = try_consume!(self, '+', '-'){
                num.push(c);
            }

            num += &self.parse_digits(radix, false)?;
        }

        Ok(TokenType::Numeric { raw: num, hint })

    }

    fn parse_string_literal(&mut self, start: char) -> Result<TokenType, LexerError>{
        let mut seen_backslash: bool = false;
        let mut literal: String = start.to_string();

        loop {
            match self.chars.peek() {
                Some(c) if *c == '\\' && !seen_backslash => {
                    seen_backslash = true;
                    literal.push(self.consume_char().unwrap());
                },
                Some(c) if *c =='"' && !seen_backslash => {
                    literal.push(self.consume_char().unwrap());
                    break Ok(TokenType::LString(literal));
                }
                Some(_c) => literal.push(self.consume_char().unwrap()),
                _ => break Ok(TokenType::LString(literal))
            }
        }
    }

    fn parse_variable(&mut self) -> Result<TokenType, LexerError> {
        let mut var: String = String::new();
        loop {
            match self.chars.peek() {
                Some(c) if c.is_ascii_alphabetic() || *c == '_' => var.push(self.consume_char().unwrap()),
                None => break Err(LexerError::MissingExpectedSymbol { 
                    expected: "<variable name>".to_string(), 
                    found: TokenType::EOF, 
                    line: self.line, 
                    col: self.col 
                }),
                _ => break Ok(TokenType::Variable(var))
            }
        }
    }

    fn parse_identifier(&mut self, start: char) -> Result<TokenType, LexerError> {
        let mut ident: String = start.to_string();
        loop {
            match self.chars.peek() {
                Some(c) if c.is_ascii_alphabetic() || *c == '_' => ident.push(self.consume_char().unwrap()),
                None => break Err(LexerError::MissingExpectedSymbol { 
                    expected: "<identifier name>".to_string(), 
                    found: TokenType::EOF, 
                    line: self.line, 
                    col: self.col 
                }),
                _ => break Ok(parse_keyword(ident))
            }
        }
    }

    fn parse_php_open_tag(&mut self, start: char) -> Result<TokenType, LexerError> {
        let mut tag: String = start.to_string();

        //TODO need to improve this, its narly
        if let Some(c) = try_consume!(self, '?') {
            tag.push(c);
            if let Some(c) = try_consume!(self, 'p') {
                tag.push(c);
                if let Some(c) = try_consume!(self, 'h') {
                    tag.push(c);
                    if let Some(c) = try_consume!(self, 'p') {
                        tag.push(c);
                        if let Some(c) = self.parse_whitespace() {
                            tag.push(c);
                            Ok(TokenType::Tag { raw: tag, kind: TagKind::PhpOpen })
                        }else{
                            Err(LexerError::UnknownSymbol { symbol: "".to_string(), line: self.line, col: self.col })
                        }
                    }else{
                        Err(LexerError::UnknownSymbol { symbol: "".to_string(), line: self.line, col: self.col })
                    }
                }else{
                    Err(LexerError::UnknownSymbol { symbol: "".to_string(), line: self.line, col: self.col })
                }
            }else{
                Err(LexerError::UnknownSymbol { symbol: "".to_string(), line: self.line, col: self.col })
            }
        }else{
            Ok(TokenType::Operator { raw:start.to_string(), kind: OperatorKind::LessThanOp })
        }
    }

    fn parse_operators(&mut self, start: char) -> Result<TokenType, LexerError> {
        let mut op: String = start.to_string();

        match start {
            '=' => {
                if let Some(c) = try_consume!(self, '>') {
                    op.push(c);
                    Ok(TokenType::Operator { raw: op, kind: OperatorKind::DoubleArrowOp })
                } else if let Some(c) = try_consume!(self, '=') {
                    op.push(c);
                    if let Some(c) = try_consume!(self, '=') {
                        op.push(c);
                        Ok(TokenType::Operator { raw: op, kind: OperatorKind::StrictEqualityOp })
                    }else{
                        Ok(TokenType::Operator { raw: op, kind: OperatorKind::EqualityOp })
                    }
                } else {
                    Ok(TokenType::Operator { raw: op, kind: OperatorKind::AssignmentOp })
                }
            },
            '-' => {
                if let Some(c) = try_consume!(self, '>') {
                    op.push(c);
                    Ok(TokenType::Operator { raw: op, kind: OperatorKind::ObjectOp })
                } else {
                    Ok(TokenType::Operator { raw: op, kind: OperatorKind::MinusOp })
                }
            },
            '+' => Ok(TokenType::Operator { raw: op, kind: OperatorKind::PlusOp }),
            '.' => Ok(TokenType::Operator { raw: op, kind: OperatorKind::ConcatOp }),
            _ => Err(LexerError::UnknownOperator { raw:op, line: self.line, col: self.col })
        }
    }

    fn consume_digit(&mut self, raw: &String) -> Result<char, LexerError>{
        match self.chars.peek() {
            None => Err(LexerError::NonNumericLiteralInvalidChar { symbol: raw.to_string(), line: self.line, col: self.col }),
            Some(c) if !c.is_digit(10) => Err(LexerError::NonNumericLiteralInvalidChar { symbol: raw.to_string(), line: self.line, col: self.col }),
            Some(c) => {
                Ok(*c)
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

    fn parse_whitespace(&mut self) -> Option<char>{
        match self.chars.peek() {
            Some(c) if c.is_whitespace() => Some(*c),
            _ => None
        }
    }

    pub fn consume_whitespace(&mut self, start: char) -> TokenType{
        let mut whitespace: String = start.to_string();

        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace(){
                break;
            }
            whitespace.push(self.consume_char().unwrap());
        }

        TokenType::Whitespace(whitespace)
    }

    fn transform_to_type(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            _ if c.is_whitespace() => Ok(self.consume_whitespace(c)),
            '<' => self.parse_php_open_tag(c),
            '(' | '[' | '{' => Ok(TokenType::Punctuation { raw:c, kind: PunctuationKind::Open(self.push_depth_map(c)) }),
            ')' | ']' | '}' => Ok(TokenType::Punctuation { raw:c, kind: PunctuationKind::Close(self.pop_depth_map(c)?) }),
            '0'..='9' => self.parse_numeric(c),
            ';' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Separator}),
            '"' => self.parse_string_literal(c),
            '=' | '-' | '+' | '.'  => self.parse_operators(c),
            _ if c.is_ascii_alphanumeric() || c == '_' => self.parse_identifier(c),
            '$' => self.parse_variable(),
            _ => Err(LexerError::UnknownSymbol { symbol: c.to_string(), line: self.line, col: self.col })
        }
    }

    pub fn next_token(&mut self) -> Result<TokenType, LexerError>{
        //self.skip_whitespace();

        if let Some(c) = self.consume_char() {
            self.transform_to_type(c)
        } else {
            Ok(TokenType::EOF)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut lex: Lexer = Lexer::new("()");
        let res: Result<TokenType, LexerError> = lex.next_token();
        assert_eq!(res.is_ok(), true);
        assert!(matches!(res.unwrap(), TokenType::Punctuation { raw: '(', kind: PunctuationKind::Close(0) }));
    }
}