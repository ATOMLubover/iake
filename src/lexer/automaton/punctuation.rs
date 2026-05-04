use crate::input::Input;
use crate::lexer::automaton::Automaton;
use crate::lexer::result::{Error as LexerError, Result as LexerResult};

#[derive(Debug, Default)]
pub struct PunctuationAutomaton {}

impl Automaton for PunctuationAutomaton {
    fn try_accept(&mut self, _: &mut [char], _: usize, input: &mut impl Input) -> LexerResult {
        // 分隔符的 regex 为 [\(\)\{\};]
        // 对应 DFA：
        // S：初始状态，接受输入 ( ) { } ;，然后转移到状态 A；接受其他输入，停机
        // A：终态，直接停机

        // 由于过于简单，此处不再使用严格的 DFA
        match input.peek() {
            Some(c) if self.acceptable(c) => {
                // 直接接受一个字符，返回对应的 Token
                // 注意，需要推进输入指针
                input.advance();
                Ok(crate::token::Token::Punctuation(c.to_string()))
            }
            Some(c) => {
                // 如果输入字符不合法，直接抛出错误
                Err(LexerError::InvalidChar(c))
            }
            None => {
                // 如果输入已经结束，抛出 EOF
                Err(LexerError::EndOfInput)
            }
        }
    }

    fn acceptable(&self, c: char) -> bool {
        matches!(c, '(' | ')' | '{' | '}' | ';')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::input::Input;
    use crate::lexer::automaton::tests::try_accept;
    use crate::token::Token;

    // 逻辑：单字符匹配。
    // 测试案例：
    // - 输入 ";"，返回 Token::Punctuation(";")
    // - 输入 "{"，返回 Token::Punctuation("{")
    // - 输入 "}"，返回 Token::Punctuation("}")
    // - 输入 "(("，返回 Token::Punctuation("(")，且指针停在第二个 '(' 上
    // - 输入 "a"，返回 Err(LexerError::InvalidChar('a'))，且指针停在 'a' 上

    #[test]
    fn accepts_semicolon() {
        let (result, _) = try_accept(";", PunctuationAutomaton::default());

        assert!(matches!(result, Ok(Token::Punctuation(ref s)) if s == ";"));
    }

    #[test]
    fn accepts_left_brace() {
        let (result, _) = try_accept("{", PunctuationAutomaton::default());

        assert!(matches!(result, Ok(Token::Punctuation(ref s)) if s == "{"));
    }

    #[test]
    fn accepts_right_brace() {
        let (result, _) = try_accept("}", PunctuationAutomaton::default());

        assert!(matches!(result, Ok(Token::Punctuation(ref s)) if s == "}"));
    }

    #[test]
    fn accepts_left_paren() {
        let (result, _) = try_accept("(", PunctuationAutomaton::default());

        assert!(matches!(result, Ok(Token::Punctuation(ref s)) if s == "("));
    }

    #[test]
    fn accepts_right_paren() {
        let (result, _) = try_accept(")", PunctuationAutomaton::default());

        assert!(matches!(result, Ok(Token::Punctuation(ref s)) if s == ")"));
    }

    #[test]
    fn stops_before_second_paren() {
        let (result, input) = try_accept("((", PunctuationAutomaton::default());

        assert!(matches!(result, Ok(Token::Punctuation(ref s)) if s == "("));
        assert_eq!(input.peek(), Some('('));
    }

    #[test]
    fn rejects_non_punctuation() {
        let (result, input) = try_accept("a", PunctuationAutomaton::default());

        assert!(matches!(result, Err(LexerError::InvalidChar('a'))));
        assert_eq!(input.peek(), Some('a'));
    }

    #[test]
    fn rejects_empty_input() {
        let (result, _) = try_accept("", PunctuationAutomaton::default());

        assert!(matches!(result, Err(LexerError::EndOfInput)));
    }
}
