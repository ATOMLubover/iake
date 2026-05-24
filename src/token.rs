#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Keyword(KeywordToken),
    Identifier(String),
    Integer(i64),
    Operator(OperatorToken),
    Punctuation(PunctuationToken),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordToken {
    I32,
    If,
    Else,
    While,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorToken {
    Equal,  // ==
    Assign, // =
    Mul,    // *
    Less,   // <
    Add,    // +
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            KeywordToken::While => write!(f, "while"),
        }
    }
}

impl std::fmt::Display for OperatorToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorToken::Equal => write!(f, "=="),
            OperatorToken::Assign => write!(f, "="),
            OperatorToken::Mul => write!(f, "*"),
            OperatorToken::Less => write!(f, "<"),
            OperatorToken::Add => write!(f, "+"),
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
    /// | 4 | KeywordWhile | `while` |
    /// | 5 | Identifier | 标识符 |
    /// | 6 | Integer | 整数 |
    /// | 7 | OperatorEq | `==` |
    /// | 8 | OperatorAssign | `=` |
    /// | 9 | OperatorMul | `*` |
    /// | 10 | OperatorLess | `<` |
    /// | 11 | OperatorAdd | `+` |
    /// | 12 | ParenLeft | `(` |
    /// | 13 | ParenRight | `)` |
    /// | 14 | BraceLeft | `{` |
    /// | 15 | BraceRight | `}` |
    /// | 16 | Semicolon | `;` |
    pub fn code(&self) -> u8 {
        match self {
            Token::Keyword(k) => match k {
                KeywordToken::I32 => 1,
                KeywordToken::If => 2,
                KeywordToken::Else => 3,
                KeywordToken::While => 4,
            },
            Token::Identifier(_) => 5,
            Token::Integer(_) => 6,
            Token::Operator(op) => match op {
                OperatorToken::Equal => 7,
                OperatorToken::Assign => 8,
                OperatorToken::Mul => 9,
                OperatorToken::Less => 10,
                OperatorToken::Add => 11,
            },
            Token::Punctuation(p) => match p {
                PunctuationToken::ParenLeft => 12,
                PunctuationToken::ParenRight => 13,
                PunctuationToken::BraceLeft => 14,
                PunctuationToken::BraceRight => 15,
                PunctuationToken::Semicolon => 16,
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
        assert_eq!(Token::Keyword(KeywordToken::While).code(), 4);
    }

    #[test]
    fn identifier_encoding() {
        assert_eq!(Token::Identifier("x".into()).code(), 5);
    }

    #[test]
    fn integer_encoding() {
        assert_eq!(Token::Integer(42).code(), 6);
    }

    #[test]
    fn operator_encoding() {
        assert_eq!(Token::Operator(OperatorToken::Equal).code(), 7);
        assert_eq!(Token::Operator(OperatorToken::Assign).code(), 8);
        assert_eq!(Token::Operator(OperatorToken::Mul).code(), 9);
        assert_eq!(Token::Operator(OperatorToken::Less).code(), 10);
        assert_eq!(Token::Operator(OperatorToken::Add).code(), 11);
    }

    #[test]
    fn punctuation_encoding() {
        assert_eq!(Token::Punctuation(PunctuationToken::ParenLeft).code(), 12);
        assert_eq!(Token::Punctuation(PunctuationToken::ParenRight).code(), 13);
        assert_eq!(Token::Punctuation(PunctuationToken::BraceLeft).code(), 14);
        assert_eq!(Token::Punctuation(PunctuationToken::BraceRight).code(), 15);
        assert_eq!(Token::Punctuation(PunctuationToken::Semicolon).code(), 16);
    }
}
