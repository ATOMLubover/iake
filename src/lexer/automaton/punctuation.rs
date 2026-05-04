use crate::input::Input;
use crate::lexer::automaton::Automaton;
use crate::lexer::result::Result as LexerResult;

#[derive(Debug, Default)]
pub struct PunctuationAutomaton {}

impl Automaton for PunctuationAutomaton {
    fn try_accept(
        &mut self,
        buf: &mut [char],
        maxlen: usize,
        input: &mut impl Input,
    ) -> LexerResult {
        todo!()
    }

    fn acceptable(&self, c: char) -> bool {
        matches!(c, '(' | ')' | '{' | '}' | ';')
    }
}

#[cfg(test)]
mod tests {
    // 逻辑：单字符匹配。
    // 测试案例：
    // - 输入 ";"，返回 Token::Punctuation(";")
    // - 输入 "{"，返回 Token::Punctuation("{")
    // - 输入 "}"，返回 Token::Punctuation("}")
}
