use crate::input::Input;
use crate::lexer::automaton::Automaton;
use crate::lexer::result::{Error as LexerError, Result as LexerResult};
use crate::token::{PunctuationToken, Token};

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
            Some(c) if matches!(c, '(' | ')' | '{' | '}' | ';') => {
                input.advance();
                match c {
                    '(' => Ok(Token::Punctuation(PunctuationToken::ParenLeft)),
                    ')' => Ok(Token::Punctuation(PunctuationToken::ParenRight)),
                    '{' => Ok(Token::Punctuation(PunctuationToken::BraceLeft)),
                    '}' => Ok(Token::Punctuation(PunctuationToken::BraceRight)),
                    ';' => Ok(Token::Punctuation(PunctuationToken::Semicolon)),
                    _ => unreachable!(),
                }
            }
            Some(c) => {
                // 已经进入 DFA 但字符不可处理
                Err(LexerError::UnexpectedChar(c))
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

    // 逻辑：单字符匹配。
    // 测试案例：
    // - 输入 ";"，返回 Token::Punctuation(Semicolon)
    // - 输入 "{"，返回 Token::Punctuation(BraceLeft)
    // - 输入 "}"，返回 Token::Punctuation(BraceRight)
    // - 输入 "("，返回 Token::Punctuation(ParenLeft)
    // - 输入 ")"，返回 Token::Punctuation(ParenRight)
    // - 输入 "(("，返回 Token::Punctuation(ParenLeft)，且指针停在第二个 '(' 上
    // - 输入 "a"，返回 Err(LexerError::UnexpectedChar('a'))，且指针停在 'a' 上

    #[test]
    fn accepts_semicolon() {
        let (result, _) = try_accept(";", PunctuationAutomaton::default());

        assert!(matches!(
            result,
            Ok(Token::Punctuation(PunctuationToken::Semicolon))
        ));
    }

    #[test]
    fn accepts_left_brace() {
        let (result, _) = try_accept("{", PunctuationAutomaton::default());

        assert!(matches!(
            result,
            Ok(Token::Punctuation(PunctuationToken::BraceLeft))
        ));
    }

    #[test]
    fn accepts_right_brace() {
        let (result, _) = try_accept("}", PunctuationAutomaton::default());

        assert!(matches!(
            result,
            Ok(Token::Punctuation(PunctuationToken::BraceRight))
        ));
    }

    #[test]
    fn accepts_left_paren() {
        let (result, _) = try_accept("(", PunctuationAutomaton::default());

        assert!(matches!(
            result,
            Ok(Token::Punctuation(PunctuationToken::ParenLeft))
        ));
    }

    #[test]
    fn accepts_right_paren() {
        let (result, _) = try_accept(")", PunctuationAutomaton::default());

        assert!(matches!(
            result,
            Ok(Token::Punctuation(PunctuationToken::ParenRight))
        ));
    }

    #[test]
    fn stops_before_second_paren() {
        let (result, input) = try_accept("((", PunctuationAutomaton::default());

        assert!(matches!(
            result,
            Ok(Token::Punctuation(PunctuationToken::ParenLeft))
        ));
        assert_eq!(input.peek(), Some('('));
    }

    #[test]
    fn rejects_non_punctuation() {
        let (result, input) = try_accept("a", PunctuationAutomaton::default());

        assert!(matches!(result, Err(LexerError::UnexpectedChar('a'))));
        assert_eq!(input.peek(), Some('a'));
    }

    #[test]
    fn rejects_empty_input() {
        let (result, _) = try_accept("", PunctuationAutomaton::default());

        assert!(matches!(result, Err(LexerError::EndOfInput)));
    }
}
