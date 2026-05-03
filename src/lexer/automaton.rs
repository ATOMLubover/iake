pub mod identifier;
pub mod integer;
pub mod operator;
pub mod punctuation;

use crate::input::Input;
use crate::lexer::result::Result;

pub trait Automaton {
    // try_accept 尝试接受输入，如果成功则返回 Ok(Token)，否则返回 Err(Error)
    fn try_accept(&mut self, buf: &mut [char], input: &mut impl Input) -> Result;
}
