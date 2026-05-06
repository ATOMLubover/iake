pub enum Token {
    Keyword(KeywordToken),
    Identifier(String),
    Integer(i64),
    Operator(OperatorToken),
    Punctuation(PunctuationToken),
}

#[derive(Debug, PartialEq, Eq)]
pub enum KeywordToken {
    I32,
    If,
    Else,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperatorToken {
    Equal,  // ==
    Assign, // =
    Mul,    // *
}

#[derive(Debug, PartialEq, Eq)]
pub enum PunctuationToken {
    ParenLeft,  // (
    ParenRight, // )
    BraceLeft,  // {
    BraceRight, // }
    Semicolon,  // ;
}

impl std::fmt::Display for KeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordToken::I32 => write!(f, "i32"),
            KeywordToken::If => write!(f, "if"),
            KeywordToken::Else => write!(f, "else"),
        }
    }
}

impl std::fmt::Display for OperatorToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorToken::Equal => write!(f, "=="),
            OperatorToken::Assign => write!(f, "="),
            OperatorToken::Mul => write!(f, "*"),
        }
    }
}

impl std::fmt::Display for PunctuationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationToken::ParenLeft => write!(f, "("),
            PunctuationToken::ParenRight => write!(f, ")"),
            PunctuationToken::BraceLeft => write!(f, "{{"),
            PunctuationToken::BraceRight => write!(f, "}}"),
            PunctuationToken::Semicolon => write!(f, ";"),
        }
    }
}

impl Token {
    /// 返回该 token 的单词编码（1 字节），用于语法分析器高效匹配。
    ///
    /// | 编码 | 名称 | Token 形式 |
    /// |------|------|-----------|
    /// | 1 | KeywordI32 | `i32` |
    /// | 2 | KeywordIf | `if` |
    /// | 3 | KeywordElse | `else` |
    /// | 4 | Identifier | 标识符 |
    /// | 5 | Integer | 整数 |
    /// | 6 | OperatorEq | `==` |
    /// | 7 | OperatorAssign | `=` |
    /// | 8 | OperatorMul | `*` |
    /// | 9 | ParenLeft | `(` |
    /// | 10 | ParenRight | `)` |
    /// | 11 | BraceLeft | `{` |
    /// | 12 | BraceRight | `}` |
    /// | 13 | Semicolon | `;` |
    pub fn code(&self) -> u8 {
        match self {
            Token::Keyword(k) => match k {
                KeywordToken::I32 => 1,
                KeywordToken::If => 2,
                KeywordToken::Else => 3,
            },
            Token::Identifier(_) => 4,
            Token::Integer(_) => 5,
            Token::Operator(op) => match op {
                OperatorToken::Equal => 6,
                OperatorToken::Assign => 7,
                OperatorToken::Mul => 8,
            },
            Token::Punctuation(p) => match p {
                PunctuationToken::ParenLeft => 9,
                PunctuationToken::ParenRight => 10,
                PunctuationToken::BraceLeft => 11,
                PunctuationToken::BraceRight => 12,
                PunctuationToken::Semicolon => 13,
            },
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(k) => write!(f, "Keyword({})", k),
            Token::Identifier(s) => write!(f, "Identifier({})", s),
            Token::Integer(i) => write!(f, "Integer({})", i),
            Token::Operator(op) => write!(f, "Operator({})", op),
            Token::Punctuation(p) => write!(f, "Punctuation({})", p),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_encoding() {
        assert_eq!(Token::Keyword(KeywordToken::I32).code(), 1);
        assert_eq!(Token::Keyword(KeywordToken::If).code(), 2);
        assert_eq!(Token::Keyword(KeywordToken::Else).code(), 3);
    }

    #[test]
    fn identifier_encoding() {
        assert_eq!(Token::Identifier("x".into()).code(), 4);
    }

    #[test]
    fn integer_encoding() {
        assert_eq!(Token::Integer(42).code(), 5);
    }

    #[test]
    fn operator_encoding() {
        assert_eq!(Token::Operator(OperatorToken::Equal).code(), 6);
        assert_eq!(Token::Operator(OperatorToken::Assign).code(), 7);
        assert_eq!(Token::Operator(OperatorToken::Mul).code(), 8);
    }

    #[test]
    fn punctuation_encoding() {
        assert_eq!(Token::Punctuation(PunctuationToken::ParenLeft).code(), 9);
        assert_eq!(Token::Punctuation(PunctuationToken::ParenRight).code(), 10);
        assert_eq!(Token::Punctuation(PunctuationToken::BraceLeft).code(), 11);
        assert_eq!(Token::Punctuation(PunctuationToken::BraceRight).code(), 12);
        assert_eq!(Token::Punctuation(PunctuationToken::Semicolon).code(), 13);
    }
}
