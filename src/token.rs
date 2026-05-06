pub enum Token {
    Keyword(String),
    Identifier(String),
    Integer(i64),
    Operator(String),
    Punctuation(String),
}

/// 单词编码表：为每种 token 分配唯一的整数编号，供语法分析器使用。
///
/// 编码范围分配：
///   1-3   关键字 (Keyword)
///   4     标识符 (Identifier)
///   5     整数   (Integer)
///   6-8   运算符 (Operator)
///   9-13  分隔符 (Punctuation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenKind {
    KeywordI32 = 1,
    KeywordIf = 2,
    KeywordElse = 3,

    Identifier = 4,

    Integer = 5,

    OperatorEq = 6,     // ==
    OperatorAssign = 7, // =
    OperatorMul = 8,    // *

    ParenLeft = 9,   // (
    ParenRight = 10, // )
    BraceLeft = 11,  // {
    BraceRight = 12, // }
    Semicolon = 13,  // ;
}

impl TokenKind {
    /// 从 Token 转换到 TokenKind。Identifier 和 Integer 携带的值不参与编码区分。
    pub fn from_token(token: &Token) -> Option<TokenKind> {
        match token {
            Token::Keyword(s) => match s.as_str() {
                "i32" => Some(TokenKind::KeywordI32),
                "if" => Some(TokenKind::KeywordIf),
                "else" => Some(TokenKind::KeywordElse),
                _ => None,
            },
            Token::Identifier(_) => Some(TokenKind::Identifier),
            Token::Integer(_) => Some(TokenKind::Integer),
            Token::Operator(s) => match s.as_str() {
                "==" => Some(TokenKind::OperatorEq),
                "=" => Some(TokenKind::OperatorAssign),
                "*" => Some(TokenKind::OperatorMul),
                _ => None,
            },
            Token::Punctuation(s) => match s.as_str() {
                "(" => Some(TokenKind::ParenLeft),
                ")" => Some(TokenKind::ParenRight),
                "{" => Some(TokenKind::BraceLeft),
                "}" => Some(TokenKind::BraceRight),
                ";" => Some(TokenKind::Semicolon),
                _ => None,
            },
        }
    }

    pub fn code(self) -> u8 {
        self as u8
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            TokenKind::KeywordI32 => "KeywordI32",
            TokenKind::KeywordIf => "KeywordIf",
            TokenKind::KeywordElse => "KeywordElse",
            TokenKind::Identifier => "Identifier",
            TokenKind::Integer => "Integer",
            TokenKind::OperatorEq => "OperatorEq(==)",
            TokenKind::OperatorAssign => "OperatorAssign(=)",
            TokenKind::OperatorMul => "OperatorMul(*)",
            TokenKind::ParenLeft => "ParenLeft(()",
            TokenKind::ParenRight => "ParenRight())",
            TokenKind::BraceLeft => "BraceLeft({)",
            TokenKind::BraceRight => "BraceRight(})",
            TokenKind::Semicolon => "Semicolon(;)",
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_encoding() {
        assert_eq!(TokenKind::from_token(&Token::Keyword("i32".into())).unwrap().code(), 1);
        assert_eq!(TokenKind::from_token(&Token::Keyword("if".into())).unwrap().code(), 2);
        assert_eq!(TokenKind::from_token(&Token::Keyword("else".into())).unwrap().code(), 3);
    }

    #[test]
    fn identifier_encoding() {
        assert_eq!(TokenKind::from_token(&Token::Identifier("x".into())).unwrap().code(), 4);
    }

    #[test]
    fn integer_encoding() {
        assert_eq!(TokenKind::from_token(&Token::Integer(42)).unwrap().code(), 5);
    }

    #[test]
    fn operator_encoding() {
        assert_eq!(TokenKind::from_token(&Token::Operator("==".into())).unwrap().code(), 6);
        assert_eq!(TokenKind::from_token(&Token::Operator("=".into())).unwrap().code(), 7);
        assert_eq!(TokenKind::from_token(&Token::Operator("*".into())).unwrap().code(), 8);
    }

    #[test]
    fn punctuation_encoding() {
        assert_eq!(TokenKind::from_token(&Token::Punctuation("(".into())).unwrap().code(), 9);
        assert_eq!(TokenKind::from_token(&Token::Punctuation(")".into())).unwrap().code(), 10);
        assert_eq!(TokenKind::from_token(&Token::Punctuation("{".into())).unwrap().code(), 11);
        assert_eq!(TokenKind::from_token(&Token::Punctuation("}".into())).unwrap().code(), 12);
        assert_eq!(TokenKind::from_token(&Token::Punctuation(";".into())).unwrap().code(), 13);
    }
}
