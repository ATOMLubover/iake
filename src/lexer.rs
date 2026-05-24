pub mod automaton;
pub mod result;

use crate::input::{Cursor, Input};
use crate::lexer::automaton::Automaton as _;
use crate::lexer::automaton::identifier::IdentifierAutomaton;
use crate::lexer::automaton::integer::IntegerAutomaton;
use crate::lexer::automaton::operator::OperatorAutomaton;
use crate::lexer::automaton::punctuation::PunctuationAutomaton;
use crate::lexer::result::{Error as LexerError, Result as LexerResult};

pub struct Lexer<I>
where
    I: Input,
{
    input: I,
    buf: [char; BUF_SIZE],
    integer_automaton: IntegerAutomaton,
    identifier_automaton: IdentifierAutomaton,
    operator_automaton: OperatorAutomaton,
    punctuation_automaton: PunctuationAutomaton,
}

const BUF_SIZE: usize = 512;

impl<T> Lexer<T>
where
    T: Input,
{
    pub fn new(input: T) -> Self {
        Self {
            input,
            buf: ['\0'; BUF_SIZE],
            integer_automaton: IntegerAutomaton::default(),
            identifier_automaton: IdentifierAutomaton::new(),
            operator_automaton: OperatorAutomaton::default(),
            punctuation_automaton: PunctuationAutomaton::default(),
        }
    }

    pub fn next_token(&mut self) -> LexerResult {
        self.sanitize();

        match self.input.peek() {
            Some(c) => self.dispatch(c),
            None => Ok(None),
        }
    }

    pub fn cursor(&self) -> Cursor {
        self.input.cursor()
    }

    fn dispatch(&mut self, c: char) -> LexerResult {
        if self.integer_automaton.acceptable(c) {
            self.integer_automaton
                .try_accept(&mut self.buf, BUF_SIZE, &mut self.input)
        } else if self.identifier_automaton.acceptable(c) {
            self.identifier_automaton
                .try_accept(&mut self.buf, BUF_SIZE, &mut self.input)
        } else if self.operator_automaton.acceptable(c) {
            self.operator_automaton
                .try_accept(&mut self.buf, BUF_SIZE, &mut self.input)
        } else if self.punctuation_automaton.acceptable(c) {
            self.punctuation_automaton
                .try_accept(&mut self.buf, BUF_SIZE, &mut self.input)
        } else {
            Err(LexerError::InvalidChar(c))
        }
    }

    fn sanitize(&mut self) {
        self.left_trim();
        while self.skip_comment() {
            self.left_trim();
        }
    }

    fn left_trim(&mut self) {
        while let Some(c) = self.input.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.input.advance();
        }
    }

    fn skip_comment(&mut self) -> bool {
        match self.input.peek() {
            Some('#') => {
                while let Some(c) = self.input.peek() {
                    self.input.advance();
                    if c == '\n' {
                        break;
                    }
                }

                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::input::test_input::TestInput;
    use crate::token::{KeywordToken, OperatorToken, PunctuationToken, Token};

    fn lexer(input: &str) -> Lexer<TestInput> {
        Lexer::new(TestInput::new(input))
    }

    #[test]
    fn whitespace_only_returns_eof() {
        let mut lexer = lexer("   ");
        assert_eq!(lexer.next_token(), Ok(None));
    }

    #[test]
    fn whitespace_before_token() {
        let mut lexer = lexer("   ;");
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::Semicolon)))
        );
    }

    #[test]
    fn newlines_and_spaces() {
        let mut lexer = lexer("  \n  \n  ;");
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::Semicolon)))
        );
    }

    #[test]
    fn invalid_char_after_whitespace() {
        let mut lexer = lexer(" @ ");
        assert!(matches!(lexer.next_token(), Err(LexerError::InvalidChar('@'))));
    }

    #[test]
    fn identifier_before_operator_no_space() {
        let mut lexer = lexer("a=1");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("a".into()))));
    }

    #[test]
    fn single_line_comment_before_token() {
        let mut lexer = lexer("# comment\na");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("a".into()))));
    }

    #[test]
    fn comment_at_eof() {
        let mut lexer = lexer("# comment");
        assert_eq!(lexer.next_token(), Ok(None));
    }

    #[test]
    fn consecutive_comment_lines() {
        let mut lexer = lexer("# c1\n# c2\na");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("a".into()))));
    }

    #[test]
    fn comment_with_leading_whitespace() {
        let mut lexer = lexer("  # comment\n  a");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("a".into()))));
    }

    #[test]
    fn whitespace_then_comment_then_eof() {
        let mut lexer = lexer("  \n  # comment\n  ");
        assert_eq!(lexer.next_token(), Ok(None));
    }

    #[test]
    fn digit_start_dispatches_to_integer_not_identifier() {
        let mut lexer = lexer("123abc");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Integer(123))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("abc".into()))));
    }

    #[test]
    fn asterisk_is_tokenized_as_operator() {
        let mut lexer = lexer("*");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Mul))));
    }

    #[test]
    fn less_than_is_tokenized_as_operator() {
        let mut lexer = lexer("<");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Less))));
    }

    #[test]
    fn plus_is_tokenized_as_operator() {
        let mut lexer = lexer("+");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Add))));
    }

    #[test]
    fn asterisk_operator_in_expression() {
        let mut lexer = lexer("a * b");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("a".into()))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Mul))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("b".into()))));
    }

    #[test]
    fn less_than_operator_in_expression() {
        let mut lexer = lexer("b < 10");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("b".into()))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Less))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Integer(10))));
    }

    #[test]
    fn plus_operator_in_expression() {
        let mut lexer = lexer("a + 1");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("a".into()))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Add))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Integer(1))));
    }

    #[test]
    fn equals_start_dispatches_to_operator_not_punctuation() {
        let mut lexer = lexer("==");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Equal))));
    }

    #[test]
    fn keyword_then_identifier() {
        let mut lexer = lexer("i32 a");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Keyword(KeywordToken::I32))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("a".into()))));
    }

    #[test]
    fn full_declaration_statement() {
        let mut lexer = lexer("i32 a = 1;");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Keyword(KeywordToken::I32))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("a".into()))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Assign))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Integer(1))));
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::Semicolon)))
        );
        assert_eq!(lexer.next_token(), Ok(None));
    }

    #[test]
    fn if_else_block() {
        let mut lexer = lexer("if (a == 1) { b = 1; } else { b = 2; }");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Keyword(KeywordToken::If))));
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::ParenLeft)))
        );
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("a".into()))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Equal))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Integer(1))));
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::ParenRight)))
        );
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::BraceLeft)))
        );
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("b".into()))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Assign))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Integer(1))));
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::Semicolon)))
        );
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::BraceRight)))
        );
        assert_eq!(lexer.next_token(), Ok(Some(Token::Keyword(KeywordToken::Else))));
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::BraceLeft)))
        );
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("b".into()))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Assign))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Integer(2))));
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::Semicolon)))
        );
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::BraceRight)))
        );
        assert_eq!(lexer.next_token(), Ok(None));
    }

    #[test]
    fn statement_with_comment() {
        let mut lexer = lexer("i32 a = 1; # inline comment\ni32 b = 2;");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Keyword(KeywordToken::I32))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Identifier("a".into()))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Operator(OperatorToken::Assign))));
        assert_eq!(lexer.next_token(), Ok(Some(Token::Integer(1))));
        assert_eq!(
            lexer.next_token(),
            Ok(Some(Token::Punctuation(PunctuationToken::Semicolon)))
        );
        assert_eq!(lexer.next_token(), Ok(Some(Token::Keyword(KeywordToken::I32))));
    }

    #[test]
    fn keyword_if_not_identifier() {
        let mut lexer = lexer("if");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Keyword(KeywordToken::If))));
    }

    #[test]
    fn keyword_else_not_identifier() {
        let mut lexer = lexer("else");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Keyword(KeywordToken::Else))));
    }

    #[test]
    fn keyword_while_not_identifier() {
        let mut lexer = lexer("while");
        assert_eq!(lexer.next_token(), Ok(Some(Token::Keyword(KeywordToken::While))));
    }
}
