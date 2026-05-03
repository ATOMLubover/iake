use crate::token::Token;

pub enum Error {}

pub type Result = std::result::Result<Token, Error>;
