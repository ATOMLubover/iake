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
        let mut state = 0;
        let mut index = 0;

        while let Some(c) = input.peek() {
            if index >= maxlen {
                return Err(LexerError::TokenTooLong(buf[..index].iter().collect()));
            }

            match state {
                0 => {
                    if !(c.is_ascii_alphabetic() || c == '_') {
                        return Err(LexerError::UnexpectedChar(c));
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut index, c);
                    state = 1;
                }
                1 => {
                    if !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                        break;
                    }

                    input.advance();
                    buf_push(buf, maxlen, &mut index, c);
                }
                _ => unreachable!(),
            }
        }

        match index {
            0 => Ok(None),
            _ => {
                let token: String = buf[..index].iter().collect();
                match token.as_str() {
                    "i32" => Ok(Some(Token::Keyword(KeywordToken::I32))),
                    "if" => Ok(Some(Token::Keyword(KeywordToken::If))),
                    "else" => Ok(Some(Token::Keyword(KeywordToken::Else))),
                    "while" => Ok(Some(Token::Keyword(KeywordToken::While))),
                    _ => Ok(Some(Token::Identifier(token))),
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

    #[test]
    fn accepts_keyword_if() {
        let (result, _) = try_accept("if", IdentifierAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Keyword(KeywordToken::If))));
    }

    #[test]
    fn accepts_keyword_else() {
        let (result, _) = try_accept("else", IdentifierAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Keyword(KeywordToken::Else))));
    }

    #[test]
    fn accepts_keyword_i32() {
        let (result, _) = try_accept("i32", IdentifierAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Keyword(KeywordToken::I32))));
    }

    #[test]
    fn accepts_identifier() {
        let (result, _) = try_accept("my_var1", IdentifierAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Identifier("my_var1".into()))));
    }

    #[test]
    fn accepts_underscore_prefix() {
        let (result, _) = try_accept("_tmp", IdentifierAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Identifier("_tmp".into()))));
    }

    #[test]
    fn stops_before_paren() {
        let (result, input) = try_accept("if(flag)", IdentifierAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Keyword(KeywordToken::If))));
        assert_eq!(input.peek(), Some('('));
    }

    #[test]
    fn rejects_non_alpha_start() {
        let (result, _) = try_accept("1var", IdentifierAutomaton::default());

        assert!(matches!(result, Err(LexerError::UnexpectedChar('1'))));
    }

    #[test]
    fn accepts_empty_input_as_eof() {
        let (result, _) = try_accept("", IdentifierAutomaton::default());

        assert_eq!(result, Ok(None));
    }
}
