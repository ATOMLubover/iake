use crate::input::Input;
use crate::lexer::automaton::Automaton;
use crate::lexer::result::Result as LexerResult;

#[derive(Debug, Default)]
pub struct OperatorAutomaton {}

impl Automaton for OperatorAutomaton {
    fn try_accept(
        &mut self,
        buf: &mut [char],
        maxlen: usize,
        input: &mut impl Input,
    ) -> LexerResult {
        todo!()
    }

    fn acceptable(&self, c: char) -> bool {
        matches!(c, '=')
    }
}

#[cfg(test)]
mod tests {
    // 逻辑：== vs =，贪婪匹配。
    // 测试案例：
    // - 输入 "="，读入 '=' 发现后位不是 '='，返回 Token::Operator("=")
    // - 输入 "=="，读入 '=' 发现后位是 '='，消费两位，返回 Token::Operator("==")
    // - 输入 "==="，返回 Token::Operator("==")，且指针停在第三个 '=' 上（后续交由下一轮解析，由 **文法分析器** 报错）
    // - 输入 "=1"，返回 Token::Operator("=")，且指针停在 '1'
}
