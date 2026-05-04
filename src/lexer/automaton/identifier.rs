use std::collections::HashSet;

use crate::input::Input;
use crate::lexer::automaton::{Automaton, buf_push};
use crate::lexer::result::Result as LexerResult;
use crate::token::Token;

#[derive(Debug, Default)]
pub struct IdentifierAutomaton {
    keywords: HashSet<&'static str>,
}

impl IdentifierAutomaton {
    pub fn new() -> Self {
        let keywords = ["if", "else", "i32"].iter().cloned().collect();

        Self { keywords }
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
        // 对应 DFA 较为简单：
        // S：初始状态，接受输入 a-zA-Z_，转移到 A；接受其他输入，停机
        // A：接受输入 a-zA-Z0-9_，停机；接受其他输入，停机
        let mut state = 0;
        let mut idx = 0;

        while let Some(c) = input.peek() {
            // 长度检查，如果已经达到 maxlen，直接停机
            if idx >= maxlen {
                return Err(crate::lexer::result::Error::TokenTooLong(
                    buf[..idx].iter().collect(),
                ));
            }

            match state {
                0 => {
                    // 状态 S
                    // 检查输入是否为字母或下划线，如果不是，直接抛出错误
                    if !self.acceptable(c) {
                        return Err(crate::lexer::result::Error::InvalidChar(c));
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut idx, c);

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
                    buf_push(buf, maxlen, &mut idx, c);
                }
                _ => unreachable!(),
            }
        }

        // 只要到达了这里，且 buf 不为空，说明我们成功接受了一个标识符或关键字
        match idx {
            0 => {
                // 没有接受到任何字符，说明输入结束了
                Err(crate::lexer::result::Error::EndOfInput)
            }
            _ => {
                // 最后检查是否是关键字
                let token: String = buf[..idx].iter().collect();

                match self.keywords.contains(token.as_str()) {
                    true => Ok(Token::Keyword(token)),
                    false => Ok(Token::Identifier(token)),
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
    use crate::input::test_input::TestInput;
    use crate::lexer::result::Error as LexerError;

    // 逻辑：[a-zA-Z_][a-zA-Z0-9_]*，匹配后查表区分
    // 测试案例：
    // - 输入 "if"，应该接受成功，返回 Token::Keyword("if")
    // - 输入 "i32"，应该接受成功，返回 Token::Keyword("i32")
    // - 输入 "my_var1"，应该接受成功，返回 Token::Identifier("my_var1")
    // - 输入 "_tmp"，应该接受成功，返回 Token::Identifier("_tmp")
    // - 输入 "if(flag)"，应该接受成功，返回 Token::Keyword("if")，且指针停在 '('
    // - 输入 "1var"，在此 DFA 匹配失败，返回 LexerError::InvalidChar('1')

    fn try_accept(input: &str) -> (LexerResult, TestInput) {
        let mut automaton = IdentifierAutomaton::new();
        let mut buf = ['\0'; 1024];
        let mut input = TestInput::new(input);

        let result = automaton.try_accept(&mut buf, 1024, &mut input);

        (result, input)
    }

    #[test]
    fn accepts_keyword_if() {
        let (result, _) = try_accept("if");

        assert!(matches!(result, Ok(Token::Keyword(ref s)) if s == "if"));
    }

    #[test]
    fn accepts_keyword_else() {
        let (result, _) = try_accept("else");

        assert!(matches!(result, Ok(Token::Keyword(ref s)) if s == "else"));
    }

    #[test]
    fn accepts_keyword_i32() {
        let (result, _) = try_accept("i32");

        assert!(matches!(result, Ok(Token::Keyword(ref s)) if s == "i32"));
    }

    #[test]
    fn accepts_identifier() {
        let (result, _) = try_accept("my_var1");

        assert!(matches!(result, Ok(Token::Identifier(ref s)) if s == "my_var1"));
    }

    #[test]
    fn accepts_underscore_prefix() {
        let (result, _) = try_accept("_tmp");

        assert!(matches!(result, Ok(Token::Identifier(ref s)) if s == "_tmp"));
    }

    #[test]
    fn stops_before_paren() {
        let (result, input) = try_accept("if(flag)");

        assert!(matches!(result, Ok(Token::Keyword(ref s)) if s == "if"));
        assert_eq!(input.peek(), Some('('));
    }

    #[test]
    fn rejects_non_alpha_start() {
        let (result, _) = try_accept("1var");

        assert!(matches!(result, Err(LexerError::InvalidChar('1'))));
    }

    #[test]
    fn rejects_empty_input() {
        let (result, _) = try_accept("");

        assert!(matches!(result, Err(LexerError::EndOfInput)));
    }
}
