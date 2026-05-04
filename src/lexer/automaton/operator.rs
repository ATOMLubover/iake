use crate::input::Input;
use crate::lexer::automaton::{Automaton, buf_push};
use crate::lexer::result::{Error as LexerError, Result as LexerResult};
use crate::token::Token;

#[derive(Debug, Default)]
pub struct OperatorAutomaton {}

impl Automaton for OperatorAutomaton {
    fn try_accept(
        &mut self,
        buf: &mut [char],
        maxlen: usize,
        input: &mut impl Input,
    ) -> LexerResult {
        // 运算符的 regex 为 ==|=
        // 对应 DFA：
        // S：初始状态，接受输入 '='，转移到 A；接受其他输入，停机
        // A：接受输入 '='，转移到 B；接受其他输入，停机
        // B：终态，直接停机
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
                    // 检查输入是否为 '='，如果不是，直接抛出错误
                    if !self.acceptable(c) {
                        return Err(LexerError::InvalidChar(c));
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut index, c);

                    // 转移到状态 A
                    state = 1;
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
                _ => unreachable!(),
            }
        }

        // 根据最终状态返回对应的 Token
        match state {
            1 | 2 => Ok(Token::Operator(buf[..index].iter().collect())),
            _ => Err(LexerError::EndOfInput),
        }
    }

    fn acceptable(&self, c: char) -> bool {
        matches!(c, '=')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::input::Input;
    use crate::lexer::automaton::tests::try_accept;

    // 逻辑：== vs =
    // 测试案例：
    // - 输入 "="，读入 '=' 发现后位不是 '='，返回 Token::Operator("=")
    // - 输入 "=="，读入 '=' 发现后位是 '='，消费两位，返回 Token::Operator("==")
    // - 输入 "==="，返回 Token::Operator("==")，且指针停在第三个 '=' 上（后续交由下一轮解析，由 **文法分析器** 报错）
    // - 输入 "=1"，返回 Token::Operator("=")，且指针停在 '1'

    #[test]
    fn accepts_single_equal() {
        let (result, input) = try_accept("=", OperatorAutomaton::default());

        assert!(matches!(result, Ok(Token::Operator(ref s)) if s == "="));
        assert_eq!(input.peek(), None);
    }

    #[test]
    fn accepts_double_equal() {
        let (result, input) = try_accept("==", OperatorAutomaton::default());

        assert!(matches!(result, Ok(Token::Operator(ref s)) if s == "=="));
        assert_eq!(input.peek(), None);
    }

    #[test]
    fn greedy_stops_before_third_equal() {
        let (result, input) = try_accept("===", OperatorAutomaton::default());

        assert!(matches!(result, Ok(Token::Operator(ref s)) if s == "=="));
        assert_eq!(input.peek(), Some('='));
    }

    #[test]
    fn stops_before_digit() {
        let (result, input) = try_accept("=1", OperatorAutomaton::default());

        assert!(matches!(result, Ok(Token::Operator(ref s)) if s == "="));
        assert_eq!(input.peek(), Some('1'));
    }

    #[test]
    fn rejects_non_operator_start() {
        let (result, _) = try_accept("a", OperatorAutomaton::default());

        assert!(matches!(result, Err(LexerError::InvalidChar('a'))));
    }

    #[test]
    fn rejects_empty_input() {
        let (result, _) = try_accept("", OperatorAutomaton::default());

        assert!(matches!(result, Err(LexerError::EndOfInput)));
    }
}
