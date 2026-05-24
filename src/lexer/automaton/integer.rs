use crate::input::Input;
use crate::lexer::automaton::{Automaton, buf_push};
use crate::lexer::result::{Error as LexerError, Result as LexerResult};
use crate::token::Token;

// IntegerAutomaton 暂时只接受 **非负整数**
#[derive(Debug, Default)]
pub struct IntegerAutomaton {}

impl IntegerAutomaton {
    pub fn to_integer(buf: &[char]) -> Option<i64> {
        let mut ans: i64 = 0;

        for &c in buf {
            if !c.is_ascii_digit() {
                return None;
            }

            ans = ans.checked_mul(10)?;
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
        let mut state = 0;
        let mut index = 0;

        while let Some(c) = input.peek() {
            if index >= maxlen {
                return Err(LexerError::TokenTooLong(buf[..index].iter().collect()));
            }

            match state {
                0 => {
                    if !self.acceptable(c) {
                        return Err(LexerError::UnexpectedChar(c));
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut index, c);

                    match c {
                        '0' => state = 1,
                        '1'..='9' => state = 2,
                        _ => unreachable!(),
                    }
                }
                1 => {
                    if c.is_ascii_digit() {
                        return Err(LexerError::UnexpectedChar(c));
                    }
                    break;
                }
                2 => {
                    if !c.is_ascii_digit() {
                        break;
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut index, c);
                }
                _ => unreachable!(),
            }
        }

        match state {
            0 => Ok(None),
            1 | 2 => match Self::to_integer(&buf[..index]) {
                Some(v) => Ok(Some(Token::Integer(v))),
                None => Err(LexerError::InvalidInteger(buf[..index].iter().collect())),
            },
            _ => unreachable!(),
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
    use crate::lexer::automaton::tests::try_accept;

    #[test]
    fn accepts_positive_integer() {
        let (result, _) = try_accept("123", IntegerAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Integer(123))));
    }

    #[test]
    fn accepts_zero() {
        let (result, _) = try_accept("0", IntegerAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Integer(0))));
    }

    #[test]
    fn rejects_integer_with_leading_zero() {
        let (result, _input) = try_accept("0123", IntegerAutomaton::default());

        assert!(matches!(result, Err(LexerError::UnexpectedChar('1'))));
    }

    #[test]
    fn rejects_non_digit_at_start() {
        let (result, _) = try_accept("abc", IntegerAutomaton::default());

        assert!(matches!(result, Err(LexerError::UnexpectedChar('a'))));
    }

    #[test]
    fn accepts_empty_input_as_eof() {
        let (result, _) = try_accept("", IntegerAutomaton::default());

        assert_eq!(result, Ok(None));
    }

    #[test]
    fn stops_before_first_non_digit() {
        let (result, input) = try_accept("123abc", IntegerAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Integer(123))));
        assert_eq!(input.peek(), Some('a'));
    }
}
