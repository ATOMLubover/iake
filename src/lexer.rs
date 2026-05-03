pub mod automaton;
pub mod result;

use crate::input::{Input, Position};
use crate::lexer::result::Result;

pub struct Lexer<I>
where
    I: Input,
{
    input: I,
}

impl<T> Lexer<T>
where
    T: Input,
{
    const BUF_SIZE: usize = 1024;

    pub fn new(input: T) -> Self {
        Self { input }
    }

    pub fn next_token(&mut self) -> Result {
        let mut buf = ['\0'; Self::BUF_SIZE];
        let start_pos = Position::default();

        unimplemented!()
    }
}
