use crate::lexer::automaton::Automaton;
use crate::lexer::result::Result as LexerResult;

#[derive(Debug, Default)]
pub struct IdentifierAutomaton {}

impl Automaton for IdentifierAutomaton {
    fn try_accept(
        &mut self,
        buf: &mut [char],
        maxlen: usize,
        input: &mut impl crate::input::Input,
    ) -> LexerResult {
        todo!()
    }

    fn acceptable(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }
}
