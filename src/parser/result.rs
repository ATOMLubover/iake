use crate::input::Cursor;
use crate::lexer::result::Error as LexerError;
use crate::token::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Lex {
        err: LexerError,
        cursor: Cursor,
    },
    UnexpectedToken {
        expected: String,
        found: Token,
        cursor: Cursor,
    },
    UnexpectedEndOfInput {
        expected: String,
        cursor: Cursor,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
