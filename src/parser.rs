pub mod result;

use crate::ast::{
    AssignStatement, BinaryOperator, Block, DeclStatement, Expression, IfStatement, Program,
    Statement, WhileStatement,
};
use crate::input::{Cursor, Input};
use crate::lexer::Lexer;
use crate::parser::result::{Error as ParserError, Result as ParserResult};
use crate::token::{KeywordToken, OperatorToken, PunctuationToken, Token};

pub struct Parser<I>
where
    I: Input,
{
    lexer: Lexer<I>,
    ahead: Option<Token>,
    cursor: Cursor,
}

impl<I> Parser<I>
where
    I: Input,
{
    pub fn new(input: I) -> Self {
        Self {
            lexer: Lexer::new(input),
            ahead: None,
            cursor: Cursor::default(),
        }
    }

    pub fn parse_program(&mut self) -> ParserResult<Program> {
        let stms = self.parse_stm_list()?;

        if self.peek_token()?.is_some() {
            return Err(self.current_error("end of input")?);
        }

        Ok(Program { stms })
    }

    fn parse_stm_list(&mut self) -> ParserResult<Vec<Statement>> {
        let mut stms = Vec::new();

        while self.starts_stm()? {
            stms.push(self.parse_stm()?);
        }

        Ok(stms)
    }

    fn parse_stm(&mut self) -> ParserResult<Statement> {
        match self.peek_token()? {
            Some(Token::Keyword(KeywordToken::I32)) => self.parse_decl_stm().map(Statement::Decl),
            Some(Token::Identifier(_)) => self.parse_assign_stm().map(Statement::Assign),
            Some(Token::Keyword(KeywordToken::If)) => self.parse_if_stm().map(Statement::If),
            Some(Token::Keyword(KeywordToken::While)) => {
                self.parse_while_stm().map(Statement::While)
            }
            _ => Err(self.current_error("start of stm")?),
        }
    }

    fn parse_decl_stm(&mut self) -> ParserResult<DeclStatement> {
        self.expect_keyword(KeywordToken::I32)?;

        let name = self.expect_identifier()?;
        self.expect_operator(OperatorToken::Assign)?;

        let init = self.parse_arith_expr()?;
        self.expect_punctuation(PunctuationToken::Semicolon)?;

        Ok(DeclStatement { name, init })
    }

    fn parse_assign_stm(&mut self) -> ParserResult<AssignStatement> {
        let name = self.expect_identifier()?;
        self.expect_operator(OperatorToken::Assign)?;
        let value = self.parse_arith_expr()?;
        self.expect_punctuation(PunctuationToken::Semicolon)?;

        Ok(AssignStatement { name, value })
    }

    fn parse_if_stm(&mut self) -> ParserResult<IfStatement> {
        self.expect_keyword(KeywordToken::If)?;
        self.expect_punctuation(PunctuationToken::ParenLeft)?;

        let cond = self.parse_bool_expr()?;
        self.expect_punctuation(PunctuationToken::ParenRight)?;

        let then_block = self.parse_block()?;
        let else_block = self.parse_else_part()?;

        Ok(IfStatement {
            cond,
            then_block,
            else_block,
        })
    }

    fn parse_else_part(&mut self) -> ParserResult<Option<Block>> {
        match self.peek_token()? {
            Some(Token::Keyword(KeywordToken::Else)) => {
                self.expect_keyword(KeywordToken::Else)?;
                self.parse_block().map(Some)
            }
            _ => Ok(None),
        }
    }

    fn parse_while_stm(&mut self) -> ParserResult<WhileStatement> {
        self.expect_keyword(KeywordToken::While)?;
        self.expect_punctuation(PunctuationToken::ParenLeft)?;
        let cond = self.parse_bool_expr()?;
        self.expect_punctuation(PunctuationToken::ParenRight)?;
        let body = self.parse_block()?;

        Ok(WhileStatement { cond, body })
    }

    fn parse_block(&mut self) -> ParserResult<Block> {
        self.expect_punctuation(PunctuationToken::BraceLeft)?;
        let stms = self.parse_stm_list()?;
        self.expect_punctuation(PunctuationToken::BraceRight)?;

        Ok(Block { stms })
    }

    fn parse_bool_expr(&mut self) -> ParserResult<Expression> {
        let left = self.parse_arith_expr()?;
        let oper = self.parse_rel_oper()?;
        let right = self.parse_arith_expr()?;

        Ok(Expression::Binary {
            oper,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_rel_oper(&mut self) -> ParserResult<BinaryOperator> {
        match self.take_token()? {
            (Some(Token::Operator(OperatorToken::Equal)), _) => Ok(BinaryOperator::Equal),
            (Some(Token::Operator(OperatorToken::Less)), _) => Ok(BinaryOperator::Less),
            (Some(found), cursor) => Err(ParserError::UnexpectedToken {
                expected: "`==` or `<`".into(),
                found,
                cursor,
            }),
            (None, cursor) => Err(ParserError::UnexpectedEndOfInput {
                expected: "`==` or `<`".into(),
                cursor,
            }),
        }
    }

    fn parse_arith_expr(&mut self) -> ParserResult<Expression> {
        let mut left = self.parse_term()?;

        while self.peek_operator(OperatorToken::Add)? {
            self.expect_operator(OperatorToken::Add)?;
            let right = self.parse_term()?;
            left = Expression::Binary {
                oper: BinaryOperator::Add,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> ParserResult<Expression> {
        let mut left = self.parse_factor()?;

        while self.peek_operator(OperatorToken::Mul)? {
            self.expect_operator(OperatorToken::Mul)?;
            let right = self.parse_factor()?;
            left = Expression::Binary {
                oper: BinaryOperator::Mul,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> ParserResult<Expression> {
        match self.take_token()? {
            (Some(Token::Identifier(name)), _) => Ok(Expression::Identifier { name }),
            (Some(Token::Integer(value)), _) => Ok(Expression::Integer { value }),
            (Some(Token::Punctuation(PunctuationToken::ParenLeft)), _) => {
                let expr = self.parse_arith_expr()?;
                self.expect_punctuation(PunctuationToken::ParenRight)?;
                Ok(expr)
            }
            (Some(found), cursor) => Err(ParserError::UnexpectedToken {
                expected: "identifier, integer, or `(`".into(),
                found,
                cursor,
            }),
            (None, cursor) => Err(ParserError::UnexpectedEndOfInput {
                expected: "identifier, integer, or `(`".into(),
                cursor,
            }),
        }
    }

    fn starts_stm(&mut self) -> ParserResult<bool> {
        Ok(matches!(
            self.peek_token()?,
            Some(Token::Keyword(KeywordToken::I32))
                | Some(Token::Identifier(_))
                | Some(Token::Keyword(KeywordToken::If))
                | Some(Token::Keyword(KeywordToken::While))
        ))
    }

    fn peek_operator(&mut self, expected: OperatorToken) -> ParserResult<bool> {
        Ok(matches!(
            self.peek_token()?,
            Some(Token::Operator(op)) if *op == expected
        ))
    }

    fn expect_keyword(&mut self, expected: KeywordToken) -> ParserResult<()> {
        match self.take_token()? {
            (Some(Token::Keyword(found)), _) if found == expected => Ok(()),
            (Some(found), cursor) => Err(ParserError::UnexpectedToken {
                expected: format!("`{}`", expected),
                found,
                cursor,
            }),
            (None, cursor) => Err(ParserError::UnexpectedEndOfInput {
                expected: format!("`{}`", expected),
                cursor,
            }),
        }
    }

    fn expect_operator(&mut self, expected: OperatorToken) -> ParserResult<()> {
        match self.take_token()? {
            (Some(Token::Operator(found)), _) if found == expected => Ok(()),
            (Some(found), cursor) => Err(ParserError::UnexpectedToken {
                expected: format!("`{}`", expected),
                found,
                cursor,
            }),
            (None, cursor) => Err(ParserError::UnexpectedEndOfInput {
                expected: format!("`{}`", expected),
                cursor,
            }),
        }
    }

    fn expect_punctuation(&mut self, expected: PunctuationToken) -> ParserResult<()> {
        match self.take_token()? {
            (Some(Token::Punctuation(found)), _) if found == expected => Ok(()),
            (Some(found), cursor) => Err(ParserError::UnexpectedToken {
                expected: format!("`{}`", expected),
                found,
                cursor,
            }),
            (None, cursor) => Err(ParserError::UnexpectedEndOfInput {
                expected: format!("`{}`", expected),
                cursor,
            }),
        }
    }

    fn expect_identifier(&mut self) -> ParserResult<String> {
        match self.take_token()? {
            (Some(Token::Identifier(name)), _) => Ok(name),
            (Some(found), cursor) => Err(ParserError::UnexpectedToken {
                expected: "identifier".into(),
                found,
                cursor,
            }),
            (None, cursor) => Err(ParserError::UnexpectedEndOfInput {
                expected: "identifier".into(),
                cursor,
            }),
        }
    }

    fn peek_token(&mut self) -> ParserResult<Option<&Token>> {
        self.fill_ahead()?;

        Ok(self.ahead.as_ref())
    }

    fn take_token(&mut self) -> ParserResult<(Option<Token>, Cursor)> {
        if self.ahead.is_none() {
            self.fill_ahead()?;
        }

        Ok((self.ahead.take(), self.cursor))
    }

    fn fill_ahead(&mut self) -> ParserResult<()> {
        if self.ahead.is_some() {
            return Ok(());
        }

        match self.lexer.next_token() {
            Ok(Some(token)) => {
                self.cursor = self.lexer.cursor();
                self.ahead = Some(token);
                Ok(())
            }
            Ok(None) => {
                self.cursor = self.lexer.cursor();
                self.ahead = None;
                Ok(())
            }
            Err(err) => Err(ParserError::Lex {
                err,
                cursor: self.lexer.cursor(),
            }),
        }
    }

    fn current_error(&mut self, expected: impl Into<String>) -> ParserResult<ParserError> {
        self.fill_ahead()?;
        let expected = expected.into();

        Ok(match &self.ahead {
            Some(token) => ParserError::UnexpectedToken {
                expected,
                found: token.clone(),
                cursor: self.cursor,
            },
            None => ParserError::UnexpectedEndOfInput {
                expected,
                cursor: self.cursor,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::input::test_input::TestInput;

    fn parse(input: &str) -> Program {
        Parser::new(TestInput::new(input))
            .parse_program()
            .expect("parser should succeed")
    }

    #[test]
    fn parses_decl_stm() {
        let program = parse("i32 a = 1;");

        assert_eq!(
            program,
            Program {
                stms: vec![Statement::Decl(DeclStatement {
                    name: "a".into(),
                    init: Expression::Integer { value: 1 },
                })],
            }
        );
    }

    #[test]
    fn parses_arith_precedence() {
        let program = parse("a = b + c * 2;");

        assert_eq!(
            program,
            Program {
                stms: vec![Statement::Assign(AssignStatement {
                    name: "a".into(),
                    value: Expression::Binary {
                        oper: BinaryOperator::Add,
                        left: Box::new(Expression::Identifier { name: "b".into() }),
                        right: Box::new(Expression::Binary {
                            oper: BinaryOperator::Mul,
                            left: Box::new(Expression::Identifier { name: "c".into() }),
                            right: Box::new(Expression::Integer { value: 2 }),
                        }),
                    },
                })],
            }
        );
    }

    #[test]
    fn parses_nested_control_flow() {
        let program = parse("if (a == 1) { while (b < 10) { b = b * 2; } } else { a = b; }");

        assert!(matches!(program.stms.as_slice(), [Statement::If(_)]));

        let Statement::If(if_stm) = &program.stms[0] else {
            unreachable!()
        };

        assert!(matches!(
            if_stm.cond,
            Expression::Binary {
                oper: BinaryOperator::Equal,
                ..
            }
        ));
        assert!(matches!(
            if_stm.then_block.stms.as_slice(),
            [Statement::While(_)]
        ));
        assert!(if_stm.else_block.is_some());
    }

    #[test]
    fn rejects_missing_semicolon() {
        let err = Parser::new(TestInput::new("a = 1"))
            .parse_program()
            .expect_err("parser should fail");

        assert!(matches!(err, ParserError::UnexpectedEndOfInput { .. }));
    }

    #[test]
    fn rejects_missing_right_paren_in_if() {
        let err = Parser::new(TestInput::new("if (a == 1 { a = 1; }"))
            .parse_program()
            .expect_err("parser should fail");

        assert!(matches!(
            err,
            ParserError::UnexpectedToken { .. } | ParserError::UnexpectedEndOfInput { .. }
        ));
    }

    #[test]
    fn rejects_missing_right_brace() {
        let err = Parser::new(TestInput::new("while (a < 1) { a = a + 1;"))
            .parse_program()
            .expect_err("parser should fail");

        assert!(matches!(err, ParserError::UnexpectedEndOfInput { .. }));
    }

    #[test]
    fn rejects_if_without_left_paren() {
        let err = Parser::new(TestInput::new("if a == 1) { a = 1; }"))
            .parse_program()
            .expect_err("parser should fail");

        assert!(matches!(err, ParserError::UnexpectedToken { .. }));
    }

    #[test]
    fn rejects_empty_assignment_value() {
        let err = Parser::new(TestInput::new("a = ;"))
            .parse_program()
            .expect_err("parser should fail");

        assert!(matches!(err, ParserError::UnexpectedToken { .. }));
    }

    #[test]
    fn rejects_bool_expr_without_rel_op() {
        let err = Parser::new(TestInput::new("while (a) { a = 1; }"))
            .parse_program()
            .expect_err("parser should fail");

        assert!(matches!(err, ParserError::UnexpectedToken { .. }));
    }

    #[test]
    fn wraps_lexer_error_with_cursor() {
        let err = Parser::new(TestInput::new("a = @;"))
            .parse_program()
            .expect_err("parser should fail");

        assert!(matches!(
            err,
            ParserError::Lex {
                err: crate::lexer::result::Error::InvalidChar('@'),
                ..
            }
        ));
    }
}
