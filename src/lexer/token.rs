use super::keywords::KeywordKind;

pub type DepthStateType = i32;

#[derive(Debug)]
pub enum PunctuationKind {
    Open(DepthStateType),
    Close(DepthStateType),
    Separator
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TagKind {
    PhpOpen,
    PhpClose
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum OperatorKind {
    AssignmentOp, // =
    EqualityOp, // ==
    StrictEqualityOp, // ===
    ConcatOp, // .
    PlusOp, // +
    MinusOp, // -
    ObjectOp, // ->
    DoubleArrowOp, // => 
    LessThanOp, // <
    LessThanEqualOp, // <=
    GreaterThanOp, // >
    GreaterThanEqualOp, // >=
    BinaryAndOp, // &&
    BinaryOrOp // ||
}

#[derive(Debug)]
pub enum NumericHint {
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
    Tag{raw: String, kind: TagKind},

    Variable(String),
    Identifier(String),
    Keyword{kind: KeywordKind},

    Char(char),

    Numeric{raw: String, hint: NumericHint },
    LString(String),

    Unkown(char),
}
