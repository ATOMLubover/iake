use crate::input::Input;
use crate::lexer::automaton::{Automaton, buf_push};
use crate::lexer::result::{Error as LexerError, Result as LexerResult};
use crate::token::{OperatorToken, Token};

#[derive(Debug, Default)]
pub struct OperatorAutomaton {}

impl Automaton for OperatorAutomaton {
    fn try_accept(
        &mut self,
        buf: &mut [char],
        maxlen: usize,
        input: &mut impl Input,
    ) -> LexerResult {
        // 运算符的 regex 为 ==|=|*|<|+
        // 对应 DFA：
        // S：初始状态，接受 '=' 转到 A；接受 '*'/'<'/'+' 转到 C；接受其他输入，停机
        // A：接受 '=' 转到 B；接受其他输入，停机，这也是一个终态
        // B：终态，直接停机
        // C：终态，直接停机
        let mut state = 0;
        let mut index = 0;
        let mut single_char_operator = None;

        while let Some(c) = input.peek() {
            if index >= maxlen {
                return Err(LexerError::TokenTooLong(buf[..index].iter().collect()));
            }

            match state {
                0 => {
                    if !matches!(c, '=' | '*' | '<' | '+') {
                        return Err(LexerError::UnexpectedChar(c));
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut index, c);

                    match c {
                        '=' => state = 1,
                        '*' => {
                            state = 3;
                            single_char_operator = Some(OperatorToken::Mul);
                        }
                        '<' => {
                            state = 3;
                            single_char_operator = Some(OperatorToken::Less);
                        }
                        '+' => {
                            state = 3;
                            single_char_operator = Some(OperatorToken::Add);
                        }
                        _ => unreachable!(),
                    }
                }
                1 => {
                    if c != '=' {
                        break;
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut index, c);
                    state = 2;
                }
                2 | 3 => break,
                _ => unreachable!(),
            }
        }

        match state {
            0 => Ok(None),
            1 => Ok(Some(Token::Operator(OperatorToken::Assign))),
            2 => Ok(Some(Token::Operator(OperatorToken::Equal))),
            3 => Ok(Some(Token::Operator(
                single_char_operator.expect("single-char operator token"),
            ))),
            _ => unreachable!(),
        }
    }

    fn acceptable(&self, c: char) -> bool {
        matches!(c, '=' | '*' | '<' | '+')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::input::Input;
    use crate::lexer::automaton::tests::try_accept;

    #[test]
    fn accepts_single_equal() {
        let (result, input) = try_accept("=", OperatorAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Operator(OperatorToken::Assign))));
        assert_eq!(input.peek(), None);
    }

    #[test]
    fn accepts_double_equal() {
        let (result, input) = try_accept("==", OperatorAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Operator(OperatorToken::Equal))));
        assert_eq!(input.peek(), None);
    }

    #[test]
    fn greedy_stops_before_third_equal() {
        let (result, input) = try_accept("===", OperatorAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Operator(OperatorToken::Equal))));
        assert_eq!(input.peek(), Some('='));
    }

    #[test]
    fn stops_before_digit() {
        let (result, input) = try_accept("=1", OperatorAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Operator(OperatorToken::Assign))));
        assert_eq!(input.peek(), Some('1'));
    }

    #[test]
    fn rejects_non_operator_start() {
        let (result, _) = try_accept("a", OperatorAutomaton::default());

        assert!(matches!(result, Err(LexerError::UnexpectedChar('a'))));
    }

    #[test]
    fn accepts_empty_input_as_eof() {
        let (result, _) = try_accept("", OperatorAutomaton::default());

        assert_eq!(result, Ok(None));
    }

    #[test]
    fn accepts_asterisk() {
        let (result, input) = try_accept("*", OperatorAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Operator(OperatorToken::Mul))));
        assert_eq!(input.peek(), None);
    }

    #[test]
    fn asterisk_stops_before_digit() {
        let (result, input) = try_accept("*1", OperatorAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Operator(OperatorToken::Mul))));
        assert_eq!(input.peek(), Some('1'));
    }

    #[test]
    fn asterisk_not_greedy() {
        let (result, input) = try_accept("**", OperatorAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Operator(OperatorToken::Mul))));
        assert_eq!(input.peek(), Some('*'));
    }

    #[test]
    fn accepts_less_than() {
        let (result, input) = try_accept("<", OperatorAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Operator(OperatorToken::Less))));
        assert_eq!(input.peek(), None);
    }

    #[test]
    fn accepts_plus() {
        let (result, input) = try_accept("+", OperatorAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Operator(OperatorToken::Add))));
        assert_eq!(input.peek(), None);
    }
}
