pub type DepthStateType = i32;

#[derive(Debug)]
pub enum PunctuationKind {
    Open(DepthStateType),
    Close(DepthStateType),
    Separator
}

#[derive(Debug)]
pub enum OperatorKind {
    Assignment,
    Concat,
    Plus,
    Minus,
    BinaryAnd,
    BinaryOr
}

#[derive(Debug)]
pub enum NumericHint {
    Any,
    Integer,
    Float
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    EOF,
    Whitespace(String),
    /** Punctuation like , . ( [ */
    Punctuation{raw: char, kind: PunctuationKind},

    Operator{raw: String, kind: OperatorKind},

    Char(char),

    Numeric{raw: String, hint: NumericHint },
    LString(String),

    Unkown(char),
}
