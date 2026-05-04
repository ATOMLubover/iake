use crate::input::Input;
use crate::lexer::automaton::{Automaton, buf_push};
use crate::lexer::result::{Error as LexerError, Result as LexerResult};
use crate::token::Token;

// IntegerAutomaton 暂时只接受 **非负整数**
#[derive(Debug, Default)]
pub struct IntegerAutomaton {}

impl IntegerAutomaton {
    // 以零拷贝的方式将 buf 中的字符转换为整数，如果 buf 中包含非数字字符，则返回 None
    pub fn to_integer(buf: &[char]) -> Option<i64> {
        let mut ans: i64 = 0;

        for &c in buf {
            if !c.is_ascii_digit() {
                // 如果遇到非数字字符，返回 None
                return None;
            }

            // 乘以 10，检查是否溢出
            ans = ans.checked_mul(10)?;
            // 加上当前数字，检查是否溢出
            ans = ans.checked_add((c as i64) - ('0' as i64))?;
        }

        Some(ans)
    }
}

impl Automaton for IntegerAutomaton {
    fn try_accept(
        &mut self,
        buf: &mut [char],
        maxlen: usize,
        input: &mut impl Input,
    ) -> LexerResult {
        // 数字的 regex 为 0|[1-9][0-9]*
        // 对应 DFA 较为简单：
        // S：初始状态，接受输入 0，转移到 A；接受输入 1-9，转移到 B；接受其他输入，停机
        // A：终态，直接停机
        // B：接受输入 0-9，停机；接受其他输入，停机
        let mut state = 0;
        let mut idx = 0;

        while let Some(c) = input.peek() {
            // 长度检查，如果已经达到 maxlen，直接停机
            if idx >= maxlen {
                return Err(LexerError::TokenTooLong(buf[..idx].iter().collect()));
            }

            match state {
                0 => {
                    // 状态 S
                    // 检查输入是否为 0-9，如果不是，直接抛出错误
                    if !self.acceptable(c) {
                        return Err(LexerError::InvalidChar(c));
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut idx, c);

                    // 根据输入的字符转移状态
                    match c {
                        '0' => state = 1,       // 转移到状态 A
                        '1'..='9' => state = 2, // 转移到状态 B
                        _ => unreachable!(),
                    }
                }
                1 => {
                    // 状态 A
                    // 此时不应该再接受任何输出，直接停机
                    break;
                }
                2 => {
                    // 状态 B
                    // 接受输入 0-9，停机；接受其他输入，停机
                    if !c.is_ascii_digit() {
                        break;
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut idx, c);
                }
                _ => unreachable!(),
            }
        }

        // 只要到达了这里，且 buf 不为空，说明我们成功接受了一个整数
        match idx {
            0 => {
                // 没有接受到任何字符，说明输入结束了
                Err(LexerError::EndOfInput)
            }
            _ => {
                // 成功接受到一个整数，转换为 Token 返回
                Ok(Token::Integer(Self::to_integer(&buf[..idx]).unwrap()))
            }
        }
    }

    fn acceptable(&self, c: char) -> bool {
        c.is_ascii_digit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::input::Input;
    use crate::input::test_input::TestInput;

    // 测试案例：
    // - 输入 "123"，应该接受成功，返回 Token::Integer(123)
    // - 输入 "0"，应该接受成功，返回 Token::Integer(0)
    // - 输入 "0123"，应该接受失败，返回 LexerError::InvalidInteger("0123")
    // - 输入 "abc"，应该接受失败，返回 LexerError::InvalidChar('a')
    // - 输入 ""，应该接受失败，返回 LexerError::EndOfInput
    // - 输入 "123abc"，应该接受成功，返回 Token::Integer(123)，并且输入指针停在 'a' 上

    fn try_accept(input: &str) -> (LexerResult, TestInput) {
        let mut automaton = IntegerAutomaton::default();
        let mut buf = ['\0'; 1024];
        let mut input = TestInput::new(input);

        let result = automaton.try_accept(&mut buf, 1024, &mut input);

        (result, input)
    }

    #[test]
    fn accepts_positive_integer() {
        let (result, _) = try_accept("123");

        assert!(matches!(result, Ok(Token::Integer(123))));
    }

    #[test]
    fn accepts_zero() {
        let (result, _) = try_accept("0");

        assert!(matches!(result, Ok(Token::Integer(0))));
    }

    #[test]
    fn rejects_integer_with_leading_zero() {
        let (result, input) = try_accept("0123");

        assert!(matches!(result, Ok(Token::Integer(0))));
        assert_eq!(input.peek(), Some('1'));
    }

    #[test]
    fn rejects_non_digit_at_start() {
        let (result, _) = try_accept("abc");

        assert!(matches!(result, Err(LexerError::InvalidChar('a'))));
    }

    #[test]
    fn rejects_empty_input() {
        let (result, _) = try_accept("");

        assert!(matches!(result, Err(LexerError::EndOfInput)));
    }

    #[test]
    fn stops_before_first_non_digit() {
        let (result, input) = try_accept("123abc");

        assert!(matches!(result, Ok(Token::Integer(123))));
        assert_eq!(input.peek(), Some('a'));
    }
}
