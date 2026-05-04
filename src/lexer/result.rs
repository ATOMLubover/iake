use crate::token::Token;

pub enum Error {
    EndOfInput,
    UnexpectedChar(char),
    InvalidChar(char),
    InvalidInteger(String),
    TokenTooLong(String),
}

pub type Result = std::result::Result<Token, Error>;
