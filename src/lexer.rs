pub mod automaton;
pub mod result;

use crate::input::{Input, Position};
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

    integer_automaton: IntegerAutomaton,
    identifier_automaton: IdentifierAutomaton,
    operator_automaton: OperatorAutomaton,
    punctuation_automaton: PunctuationAutomaton,
}

impl<T> Lexer<T>
where
    T: Input,
{
    const BUF_SIZE: usize = 1024;

    pub fn new(input: T) -> Self {
        Self {
            input,
            integer_automaton: IntegerAutomaton::default(),
            identifier_automaton: IdentifierAutomaton::default(),
            operator_automaton: OperatorAutomaton::default(),
            punctuation_automaton: PunctuationAutomaton::default(),
        }
    }

    pub fn next_token(&mut self) -> LexerResult {
        let mut buf = ['\0'; Self::BUF_SIZE];

        self.left_trim();

        match self.input.peek() {
            Some(c) => {
                // 此处需要严格按照优先级顺序判断
                if self.integer_automaton.acceptable(c) {
                    self.integer_automaton
                        .try_accept(&mut buf, Self::BUF_SIZE, &mut self.input)
                } else if self.identifier_automaton.acceptable(c) {
                    self.identifier_automaton
                        .try_accept(&mut buf, Self::BUF_SIZE, &mut self.input)
                } else if self.operator_automaton.acceptable(c) {
                    self.operator_automaton
                        .try_accept(&mut buf, Self::BUF_SIZE, &mut self.input)
                } else if self.punctuation_automaton.acceptable(c) {
                    self.punctuation_automaton
                        .try_accept(&mut buf, Self::BUF_SIZE, &mut self.input)
                } else {
                    Err(LexerError::UnexpectedChar(c))
                }
            }
            None => Err(LexerError::EndOfInput),
        }
    }

    pub fn current_position(&self) -> Position {
        self.input.current_position()
    }

    fn left_trim(&mut self) {
        while let Some(c) = self.input.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.input.advance();
        }
    }
}
