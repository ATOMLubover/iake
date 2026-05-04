pub mod identifier;
pub mod integer;
pub mod operator;
pub mod punctuation;

use crate::input::Input;
use crate::lexer::result::Result as LexerResult;

pub trait Automaton {
    // try_accept 尝试接受输入，如果成功则返回 Ok(Token)，否则返回 Err(Error)
    fn try_accept(
        &mut self,
        buf: &mut [char],
        maxlen: usize,
        input: &mut impl Input,
    ) -> LexerResult;

    // acceptable 判断一个字符是否可以被当前自动机接受，如果可以接受则返回 true，否则返回 false
    fn acceptable(&self, c: char) -> bool;
}

fn buf_push(buf: &mut [char], maxlen: usize, idx: &mut usize, c: char) {
    if *idx >= maxlen {
        return;
    }

    buf[*idx] = c;
    *idx += 1;
}

#[cfg(test)]
mod tests {
    use crate::input::test_input::TestInput;
    use crate::lexer::result::Result as LexerResult;

    pub fn try_accept<A>(input: &str, mut automaton: A) -> (LexerResult, TestInput)
    where
        A: crate::lexer::automaton::Automaton,
    {
        let mut buf = ['\0'; 1024];
        let mut input = TestInput::new(input);

        let result = automaton.try_accept(&mut buf, 1024, &mut input);

        (result, input)
    }
}
