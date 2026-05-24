use std::fs;
use std::path::PathBuf;

use iake::input::file_input::FileInput;
use iake::lexer::Lexer;
use iake::lexer::result::Error as LexerError;
use iake::token::{OperatorToken, Token};

struct TokenInfo {
    line: usize,
    col: usize,
    text: String,
}

fn main() {
    for i in 1..=3 {
        let input_path = format!("tests/example/input-{}.txt", i);
        let output_path = format!("tests/example/output-{}.txt", i);
        process_file(&input_path, &output_path);
    }
}

fn process_file(input_path: &str, output_path: &str) {
    let input = fs::read_to_string(input_path).expect("Failed to read input file");
    let input_lines: Vec<&str> = input.lines().collect();

    let mut lexer = Lexer::new(FileInput::new(PathBuf::from(input_path)));

    let mut tokens: Vec<TokenInfo> = Vec::new();
    let mut error_info: Option<String> = None;

    loop {
        match lexer.next_token() {
            Ok(Some(token)) => {
                let cur = lexer.cursor();
                let col = cur.column.saturating_sub(token_len(&token));
                tokens.push(TokenInfo {
                    line: cur.line,
                    col,
                    text: format_token(&token),
                });
            }
            Ok(None) => break,
            Err(e) => {
                let cur = lexer.cursor();
                error_info = Some(format!(
                    "(line {}, col {}): {}",
                    cur.line,
                    cur.column,
                    format_error(&e)
                ));
                break;
            }
        }
    }

    let output = match error_info {
        Some(err) => err,
        None => {
            let mut lines: Vec<String> = Vec::new();
            for (i, _) in input_lines.iter().enumerate() {
                let line_tokens: Vec<&TokenInfo> = tokens.iter().filter(|t| t.line == i).collect();
                if line_tokens.is_empty() {
                    continue;
                }
                let indent = line_tokens[0].col;
                lines.push(format!(
                    "{}{}",
                    " ".repeat(indent),
                    line_tokens
                        .iter()
                        .map(|t| t.text.as_str())
                        .collect::<String>()
                ));
            }
            lines.join("\n")
        }
    };

    fs::write(output_path, output).expect("Failed to write output file");
}

fn format_error(err: &LexerError) -> String {
    match err {
        LexerError::UnexpectedChar(c) => format!("UnexpectedChar('{}')", c),
        LexerError::InvalidChar(c) => format!("InvalidChar('{}')", c),
        LexerError::InvalidInteger(s) => format!("InvalidInteger(\"{}\")", s),
        LexerError::TokenTooLong(s) => format!("TokenTooLong(\"{}\")", s),
    }
}

fn token_len(token: &Token) -> usize {
    use iake::token::KeywordToken;

    match token {
        Token::Keyword(k) => match k {
            KeywordToken::I32 => 3,
            KeywordToken::If => 2,
            KeywordToken::Else => 4,
            KeywordToken::While => 5,
        },
        Token::Identifier(s) => s.len(),
        Token::Integer(n) => n.to_string().len(),
        Token::Operator(op) => match op {
            OperatorToken::Equal => 2,
            OperatorToken::Assign => 1,
            OperatorToken::Mul => 1,
            OperatorToken::Less => 1,
            OperatorToken::Add => 1,
        },
        Token::Punctuation(_) => 1,
    }
}

fn format_token(token: &Token) -> String {
    match token {
        Token::Keyword(k) => format!("<Keyword, {}>", k),
        Token::Identifier(s) => format!("<Identifier, {}>", s),
        Token::Integer(n) => format!("<Integer, {}>", n),
        Token::Operator(op) => format!("<Operator, {}>", op),
        Token::Punctuation(p) => format!("<Punctuation, {}>", p),
    }
}
