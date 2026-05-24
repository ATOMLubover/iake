use crate::token::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnexpectedChar(char),
    InvalidChar(char),
    InvalidInteger(String),
    TokenTooLong(String),
}

pub type Result = std::result::Result<Option<Token>, Error>;
