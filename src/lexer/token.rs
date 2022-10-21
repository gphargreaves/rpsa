pub type DepthStateType = i32;

#[derive(Debug)]
pub enum PunctuationKind {
    Open(DepthStateType),
    Close(DepthStateType)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    EOF,
    Whitespace(String),
    /** Punctuation like , . ( [ */
    Punctuation{raw: char, kind: PunctuationKind},

    Operator(String),

    Char(char),

    Numeric(String),

    Unkown(char),
}
