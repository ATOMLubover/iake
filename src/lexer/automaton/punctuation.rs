use crate::input::Input;
use crate::lexer::automaton::Automaton;
use crate::lexer::result::{Error as LexerError, Result as LexerResult};
use crate::token::{PunctuationToken, Token};

#[derive(Debug, Default)]
pub struct PunctuationAutomaton {}

impl Automaton for PunctuationAutomaton {
    fn try_accept(&mut self, _: &mut [char], _: usize, input: &mut impl Input) -> LexerResult {
        match input.peek() {
            Some(c) if matches!(c, '(' | ')' | '{' | '}' | ';') => {
                input.advance();
                match c {
                    '(' => Ok(Some(Token::Punctuation(PunctuationToken::ParenLeft))),
                    ')' => Ok(Some(Token::Punctuation(PunctuationToken::ParenRight))),
                    '{' => Ok(Some(Token::Punctuation(PunctuationToken::BraceLeft))),
                    '}' => Ok(Some(Token::Punctuation(PunctuationToken::BraceRight))),
                    ';' => Ok(Some(Token::Punctuation(PunctuationToken::Semicolon))),
                    _ => unreachable!(),
                }
            }
            Some(c) => Err(LexerError::UnexpectedChar(c)),
            None => Ok(None),
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

    #[test]
    fn accepts_semicolon() {
        let (result, _) = try_accept(";", PunctuationAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Punctuation(PunctuationToken::Semicolon))));
    }

    #[test]
    fn accepts_left_brace() {
        let (result, _) = try_accept("{", PunctuationAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Punctuation(PunctuationToken::BraceLeft))));
    }

    #[test]
    fn accepts_right_brace() {
        let (result, _) = try_accept("}", PunctuationAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Punctuation(PunctuationToken::BraceRight))));
    }

    #[test]
    fn accepts_left_paren() {
        let (result, _) = try_accept("(", PunctuationAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Punctuation(PunctuationToken::ParenLeft))));
    }

    #[test]
    fn accepts_right_paren() {
        let (result, _) = try_accept(")", PunctuationAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Punctuation(PunctuationToken::ParenRight))));
    }

    #[test]
    fn stops_before_second_paren() {
        let (result, input) = try_accept("((", PunctuationAutomaton::default());

        assert_eq!(result, Ok(Some(Token::Punctuation(PunctuationToken::ParenLeft))));
        assert_eq!(input.peek(), Some('('));
    }

    #[test]
    fn rejects_non_punctuation() {
        let (result, input) = try_accept("a", PunctuationAutomaton::default());

        assert!(matches!(result, Err(LexerError::UnexpectedChar('a'))));
        assert_eq!(input.peek(), Some('a'));
    }

    #[test]
    fn accepts_empty_input_as_eof() {
        let (result, _) = try_accept("", PunctuationAutomaton::default());

        assert_eq!(result, Ok(None));
    }
}
