pub struct Token {
        token_type: String,
        value: String,
        line: usize,
        col: usize
    }

impl Token {
    pub fn new(token_type: &str, value: &str, line: usize, col: usize) -> Token {
        return Token{ token_type: String::from(token_type), value: String::from(value), line, col}
    }

    pub fn get_token_type(&self) -> &str{
        return self.token_type.as_str();
    }

    pub fn get_value(&self) -> &str{
        return self.value.as_str();
    }

    pub fn get_line(&self) -> usize{
        return self.line;
    }

    pub fn get_col(&self) -> usize{
        return self.col;
    }
}
