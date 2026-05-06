use crate::input::Input;
use crate::lexer::automaton::{Automaton, buf_push};
use crate::lexer::result::{Error as LexerError, Result as LexerResult};
use crate::token::{KeywordToken, Token};

#[derive(Debug, Default)]
pub struct IdentifierAutomaton {}

impl IdentifierAutomaton {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Automaton for IdentifierAutomaton {
    fn try_accept(
        &mut self,
        buf: &mut [char],
        maxlen: usize,
        input: &mut impl Input,
    ) -> LexerResult {
        // 标识符 & 关键字的 regex 为 [a-zA-Z\_][a-zA-Z0-9\_]*
        // 对应 DFA：
        // S：初始状态，接受输入 a-zA-Z_，转移到 A；接受其他输入，停机
        // A：接受输入 a-zA-Z0-9_，停机；接受其他输入，停机
        let mut state = 0;
        let mut index = 0;

        while let Some(c) = input.peek() {
            // 长度检查，如果已经达到 maxlen，直接停机
            if index >= maxlen {
                return Err(LexerError::TokenTooLong(buf[..index].iter().collect()));
            }

            match state {
                0 => {
                    // 状态 S
                    // 检查输入是否为字母或下划线，如果不是，直接抛出错误
                    if !(c.is_ascii_alphabetic() || c == '_') {
                        return Err(LexerError::UnexpectedChar(c));
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut index, c);

                    // 转移到状态 A
                    state = 1;
                }
                1 => {
                    // 状态 A
                    // 检查输入是否为字母、数字或下划线，如果不是，停机
                    if !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                        // 停机
                        break;
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut index, c);
                }
                _ => unreachable!(),
            }
        }

        // 只要到达了这里，且 buf 不为空，说明我们成功接受了一个标识符或关键字
        match index {
            0 => {
                // 没有接受到任何字符，说明输入结束了
                Err(LexerError::EndOfInput)
            }
            _ => {
                let token: String = buf[..index].iter().collect();
                match token.as_str() {
                    "i32" => Ok(Token::Keyword(KeywordToken::I32)),
                    "if" => Ok(Token::Keyword(KeywordToken::If)),
                    "else" => Ok(Token::Keyword(KeywordToken::Else)),
                    _ => Ok(Token::Identifier(token)),
                }
            }
        }
    }

    fn acceptable(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::input::Input;
    use crate::lexer::automaton::tests::try_accept;
    use crate::lexer::result::Error as LexerError;

    // 逻辑：[a-zA-Z_][a-zA-Z0-9_]*，匹配后查表区分
    // 测试案例：
    // - 输入 "if"，应该接受成功，返回 Token::Keyword(If)
    // - 输入 "else"，应该接受成功，返回 Token::Keyword(Else)
    // - 输入 "i32"，应该接受成功，返回 Token::Keyword(I32)
    // - 输入 "my_var1"，应该接受成功，返回 Token::Identifier("my_var1")
    // - 输入 "_tmp"，应该接受成功，返回 Token::Identifier("_tmp")
    // - 输入 "if(flag)"，应该接受成功，返回 Token::Keyword(If)，且指针停在 '('
    // - 输入 "1var"，在此 DFA 匹配失败，返回 LexerError::UnexpectedChar('1')

    #[test]
    fn accepts_keyword_if() {
        let (result, _) = try_accept("if", IdentifierAutomaton::default());

        assert!(matches!(result, Ok(Token::Keyword(KeywordToken::If))));
    }

    #[test]
    fn accepts_keyword_else() {
        let (result, _) = try_accept("else", IdentifierAutomaton::default());

        assert!(matches!(result, Ok(Token::Keyword(KeywordToken::Else))));
    }

    #[test]
    fn accepts_keyword_i32() {
        let (result, _) = try_accept("i32", IdentifierAutomaton::default());

        assert!(matches!(result, Ok(Token::Keyword(KeywordToken::I32))));
    }

    #[test]
    fn accepts_identifier() {
        let (result, _) = try_accept("my_var1", IdentifierAutomaton::default());

        assert!(matches!(result, Ok(Token::Identifier(ref s)) if s == "my_var1"));
    }

    #[test]
    fn accepts_underscore_prefix() {
        let (result, _) = try_accept("_tmp", IdentifierAutomaton::default());

        assert!(matches!(result, Ok(Token::Identifier(ref s)) if s == "_tmp"));
    }

    #[test]
    fn stops_before_paren() {
        let (result, input) = try_accept("if(flag)", IdentifierAutomaton::default());

        assert!(matches!(result, Ok(Token::Keyword(KeywordToken::If))));
        assert_eq!(input.peek(), Some('('));
    }

    #[test]
    fn rejects_non_alpha_start() {
        let (result, _) = try_accept("1var", IdentifierAutomaton::default());

        assert!(matches!(result, Err(LexerError::UnexpectedChar('1'))));
    }

    #[test]
    fn rejects_empty_input() {
        let (result, _) = try_accept("", IdentifierAutomaton::default());

        assert!(matches!(result, Err(LexerError::EndOfInput)));
    }
}
