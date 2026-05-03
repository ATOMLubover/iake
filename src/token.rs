pub enum Token {
    Keyword(String),
    Identifier(String),
    Integer(i64),
    Operator(String),
    Punctuation(String),
}
