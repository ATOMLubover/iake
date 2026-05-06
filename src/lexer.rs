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
        // 预处理，先去掉空白字符和注释
        self.sanitize();

        match self.input.peek() {
            Some(c) => self.dispatch(c),
            None => Err(LexerError::EndOfInput),
        }
    }

    pub fn cursor(&self) -> Cursor {
        self.input.cursor()
    }

    fn dispatch(&mut self, c: char) -> LexerResult {
        // 此处需要严格按照优先级顺序判断
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

    // left_trim 会一直推进输入指针，直到遇到第一个非空白字符或输入结束
    fn left_trim(&mut self) {
        while let Some(c) = self.input.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.input.advance();
        }
    }

    // skip_comment 会检查当前输入是否为注释的开
    // 如果是，则一直推进输入指针直到行尾或输入结束
    // 返回值表示是否跳过了注释
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
    use crate::token::Token;

    // 逻辑：Lexer 集成测试，覆盖空白跳过、注释跳过、调度优先级、多 token 序列、关键词识别、异常处理
    // 测试案例：
    // - 输入 "   "，纯空白，返回 LexerError::EndOfInput
    // - 输入 "   ;"，前导空白后接分隔符，返回 Token::Punctuation(";")
    // - 输入 "  \n  \n  ;"，多行空白混换行，返回 Token::Punctuation(";")
    // - 输入 " @ "，异常字符，返回 LexerError::InvalidChar('@')
    // - 输入 "a=1"，无空格标识符接算式，返回 Token::Identifier("a")，指针停在 '='
    // - 输入 "# comment\na"，单行注释后接标识符，返回 Token::Identifier("a")
    // - 输入 "# comment"，只有注释无 token，返回 LexerError::EndOfInput
    // - 输入 "# c1\n# c2\na"，连续多行注释，返回 Token::Identifier("a")
    // - 输入 "  # comment\n  a"，前导空白后注释再接标识符，返回 Token::Identifier("a")
    // - 输入 "  \n  # comment\n  "，空白加注释加空白，返回 LexerError::EndOfInput
    // - 输入 "123abc"，数字优先走 IntegerAutomaton，返回 Token::Integer(123)，再返回 Token::Identifier("abc")
    // - 输入 "=="，优先走 OperatorAutomaton，返回 Token::Operator("==")
    // - 输入 "*"，返回 Token::Operator("*")
    // - 输入 "a * b"，乘号两侧有空格的表达式
    // - 输入 "i32 a"，关键词接标识符，返回 Token::Keyword("i32")、Token::Identifier("a")
    // - 输入 "i32 a = 1;"，完整声明语句
    // - 输入 "if (a == 1) { b = 1; } else { b = 2; }"，完整 if-else 块
    // - 输入 "i32 a = 1; # inline comment\ni32 b = 2;"，行内注释跨语句
    // - 输入 "if"，返回 Token::Keyword("if")
    // - 输入 "else"，返回 Token::Keyword("else")

    fn lexer(input: &str) -> Lexer<TestInput> {
        Lexer::new(TestInput::new(input))
    }

    #[test]
    fn whitespace_only_returns_eof() {
        let mut lexer = lexer("   ");
        assert!(matches!(lexer.next_token(), Err(LexerError::EndOfInput)));
    }

    #[test]
    fn whitespace_before_token() {
        let mut lexer = lexer("   ;");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == ";"
        ));
    }

    #[test]
    fn newlines_and_spaces() {
        let mut lexer = lexer("  \n  \n  ;");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == ";"
        ));
    }

    #[test]
    fn invalid_char_after_whitespace() {
        let mut lexer = lexer(" @ ");
        assert!(matches!(
            lexer.next_token(),
            Err(LexerError::InvalidChar('@'))
        ));
    }

    #[test]
    fn identifier_before_operator_no_space() {
        let mut lexer = lexer("a=1");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "a"
        ));
    }

    #[test]
    fn single_line_comment_before_token() {
        let mut lexer = lexer("# comment\na");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "a"
        ));
    }

    #[test]
    fn comment_at_eof() {
        let mut lexer = lexer("# comment");
        assert!(matches!(lexer.next_token(), Err(LexerError::EndOfInput)));
    }

    #[test]
    fn consecutive_comment_lines() {
        let mut lexer = lexer("# c1\n# c2\na");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "a"
        ));
    }

    #[test]
    fn comment_with_leading_whitespace() {
        let mut lexer = lexer("  # comment\n  a");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "a"
        ));
    }

    #[test]
    fn whitespace_then_comment_then_eof() {
        let mut lexer = lexer("  \n  # comment\n  ");
        assert!(matches!(lexer.next_token(), Err(LexerError::EndOfInput)));
    }

    #[test]
    fn digit_start_dispatches_to_integer_not_identifier() {
        let mut lexer = lexer("123abc");
        assert!(matches!(lexer.next_token(), Ok(Token::Integer(123))));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "abc"
        ));
    }

    #[test]
    fn asterisk_is_tokenized_as_operator() {
        let mut lexer = lexer("*");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Operator(ref s)) if s == "*"
        ));
    }

    #[test]
    fn asterisk_operator_in_expression() {
        let mut lexer = lexer("a * b");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "a"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Operator(ref s)) if s == "*"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "b"
        ));
    }

    #[test]
    fn equals_start_dispatches_to_operator_not_punctuation() {
        let mut lexer = lexer("==");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Operator(ref s)) if s == "=="
        ));
    }

    #[test]
    fn keyword_then_identifier() {
        let mut lexer = lexer("i32 a");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Keyword(ref s)) if s == "i32"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "a"
        ));
    }

    #[test]
    fn full_declaration_statement() {
        let mut lexer = lexer("i32 a = 1;");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Keyword(ref s)) if s == "i32"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "a"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Operator(ref s)) if s == "="
        ));
        assert!(matches!(lexer.next_token(), Ok(Token::Integer(1))));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == ";"
        ));
        assert!(matches!(lexer.next_token(), Err(LexerError::EndOfInput)));
    }

    #[test]
    fn if_else_block() {
        let mut lexer = lexer("if (a == 1) { b = 1; } else { b = 2; }");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Keyword(ref s)) if s == "if"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == "("
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "a"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Operator(ref s)) if s == "=="
        ));
        assert!(matches!(lexer.next_token(), Ok(Token::Integer(1))));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == ")"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == "{"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "b"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Operator(ref s)) if s == "="
        ));
        assert!(matches!(lexer.next_token(), Ok(Token::Integer(1))));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == ";"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == "}"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Keyword(ref s)) if s == "else"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == "{"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "b"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Operator(ref s)) if s == "="
        ));
        assert!(matches!(lexer.next_token(), Ok(Token::Integer(2))));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == ";"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == "}"
        ));
        assert!(matches!(lexer.next_token(), Err(LexerError::EndOfInput)));
    }

    #[test]
    fn statement_with_comment() {
        let mut lexer = lexer("i32 a = 1; # inline comment\ni32 b = 2;");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Keyword(ref s)) if s == "i32"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Identifier(ref s)) if s == "a"
        ));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Operator(ref s)) if s == "="
        ));
        assert!(matches!(lexer.next_token(), Ok(Token::Integer(1))));
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Punctuation(ref s)) if s == ";"
        ));
        // 注释被跳过，接下来是下一行的 i32
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Keyword(ref s)) if s == "i32"
        ));
    }

    #[test]
    fn keyword_if_not_identifier() {
        let mut lexer = lexer("if");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Keyword(ref s)) if s == "if"
        ));
    }

    #[test]
    fn keyword_else_not_identifier() {
        let mut lexer = lexer("else");
        assert!(matches!(
            lexer.next_token(),
            Ok(Token::Keyword(ref s)) if s == "else"
        ));
    }
}
