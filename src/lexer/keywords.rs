use super::token::*;

#[derive(Debug)]
pub enum KeywordKind {
    Public,
    Private,
    Protected,
    Class,
    Function,
    Static,
    Namespace,
    Echo
}

pub fn parse_keyword(identifier: String) -> TokenType {
    match identifier.as_str() {
        "public" => TokenType::Keyword { kind: KeywordKind::Public },
        "private" => TokenType::Keyword { kind: KeywordKind::Private },
        "protected" => TokenType::Keyword { kind: KeywordKind::Protected },
        "class" => TokenType::Keyword { kind: KeywordKind::Class },
        "function" => TokenType::Keyword { kind: KeywordKind::Function },
        "static" => TokenType::Keyword { kind: KeywordKind::Static },
        "namespace" => TokenType::Keyword { kind: KeywordKind::Namespace },
        "echo" => TokenType::Keyword { kind: KeywordKind::Echo },
        _ => TokenType::Identifier(identifier)
    }
}