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
        // 运算符的 regex 为 ==|=|*
        // 对应 DFA：
        // S：初始状态，接受输入 '='，转移到 A；接受输入 '*'，转移到 C；接受其他输入，停机
        // A：接受输入 '='，转移到 B；接受其他输入，停机，这也是一个终态
        // B：终态，直接停机
        // C：终态，直接停机
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
                    // 检查输入是否为 '=' 或 '*'，如果不是，直接抛出错误
                    if !matches!(c, '=' | '*' | '<' | '+') {
                        return Err(LexerError::UnexpectedChar(c));
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut index, c);

                    match c {
                        '=' => {
                            // 转移到状态 A
                            state = 1;
                        }
                        '*' | '<' | '+' => {
                            // 转移到状态 C
                            state = 3;
                        }
                        _ => unreachable!(),
                    }
                }
                1 => {
                    // 状态 A
                    // 检查输入是否为 '='，如果是，接受并转移到状态 B；如果不是，停机
                    if c != '=' {
                        break;
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut index, c);

                    // 转移到状态 B
                    state = 2;
                }
                2 => {
                    // 状态 B 是终态，直接停机
                    break;
                }
                3 => {
                    // 状态 C 是终态，直接停机
                    break;
                }
                _ => unreachable!(),
            }
        }

        // 根据最终状态返回对应的 Token
        match state {
            1 => Ok(Token::Operator(OperatorToken::Assign)),
            2 => Ok(Token::Operator(OperatorToken::Equal)),
            3 => Ok(Token::Operator(OperatorToken::Mul)),
            _ => Err(LexerError::EndOfInput),
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

    // 逻辑：== vs = vs *
    // 测试案例：
    // - 输入 "="，读入 '=' 发现后位不是 '='，返回 Token::Operator(Assign)
    // - 输入 "=="，读入 '=' 发现后位是 '='，消费两位，返回 Token::Operator(Equal)
    // - 输入 "==="，返回 Token::Operator(Equal)，且指针停在第三个 '=' 上
    // - 输入 "=1"，返回 Token::Operator(Assign)，且指针停在 '1'
    // - 输入 "*"，读入 '*' 后直接停机，返回 Token::Operator(Mul)
    // - 输入 "*1"，返回 Token::Operator(Mul)，且指针停在 '1'
    // - 输入 "**"，返回 Token::Operator(Mul)，且指针停在第二个 '*' 上

    #[test]
    fn accepts_single_equal() {
        let (result, input) = try_accept("=", OperatorAutomaton::default());

        assert!(matches!(result, Ok(Token::Operator(OperatorToken::Assign))));
        assert_eq!(input.peek(), None);
    }

    #[test]
    fn accepts_double_equal() {
        let (result, input) = try_accept("==", OperatorAutomaton::default());

        assert!(matches!(result, Ok(Token::Operator(OperatorToken::Equal))));
        assert_eq!(input.peek(), None);
    }

    #[test]
    fn greedy_stops_before_third_equal() {
        let (result, input) = try_accept("===", OperatorAutomaton::default());

        assert!(matches!(result, Ok(Token::Operator(OperatorToken::Equal))));
        assert_eq!(input.peek(), Some('='));
    }

    #[test]
    fn stops_before_digit() {
        let (result, input) = try_accept("=1", OperatorAutomaton::default());

        assert!(matches!(result, Ok(Token::Operator(OperatorToken::Assign))));
        assert_eq!(input.peek(), Some('1'));
    }

    #[test]
    fn rejects_non_operator_start() {
        let (result, _) = try_accept("a", OperatorAutomaton::default());

        assert!(matches!(result, Err(LexerError::UnexpectedChar('a'))));
    }

    #[test]
    fn rejects_empty_input() {
        let (result, _) = try_accept("", OperatorAutomaton::default());

        assert!(matches!(result, Err(LexerError::EndOfInput)));
    }

    // * 是单字符运算符，读到即止
    #[test]
    fn accepts_asterisk() {
        let (result, input) = try_accept("*", OperatorAutomaton::default());

        assert!(matches!(result, Ok(Token::Operator(OperatorToken::Mul))));
        assert_eq!(input.peek(), None);
    }

    #[test]
    fn asterisk_stops_before_digit() {
        let (result, input) = try_accept("*1", OperatorAutomaton::default());

        assert!(matches!(result, Ok(Token::Operator(OperatorToken::Mul))));
        assert_eq!(input.peek(), Some('1'));
    }

    #[test]
    fn asterisk_not_greedy() {
        let (result, input) = try_accept("**", OperatorAutomaton::default());

        assert!(matches!(result, Ok(Token::Operator(OperatorToken::Mul))));
        assert_eq!(input.peek(), Some('*'));
    }
}
