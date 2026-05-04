use crate::lexer::automaton::Automaton;
use crate::lexer::result::Result as LexerResult;

#[derive(Debug, Default)]
pub struct PunctuationAutomaton {}

impl Automaton for PunctuationAutomaton {
    fn try_accept(
        &mut self,
        buf: &mut [char],
        maxlen: usize,
        input: &mut impl crate::input::Input,
    ) -> LexerResult {
        todo!()
    }

    fn acceptable(&self, c: char) -> bool {
        matches!(c, '(' | ')' | '{' | '}' | ';')
    }
}
