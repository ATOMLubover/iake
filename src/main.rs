use std::path::PathBuf;
use std::{fs, iter};

use iake::input::file_input::FileInput;
use iake::lexer::Lexer;
use iake::lexer::result::Error as LexerError;
use iake::token::Token;

struct TokenInfo {
    line: usize,
    col: usize,
    text: String,
}

fn main() {
    let input_path = "tests/example/input.txt";
    let input = fs::read_to_string(input_path).expect("Failed to read input file");
    let input_lines: Vec<&str> = input.lines().collect();

    let mut lexer = Lexer::new(FileInput::new(PathBuf::from(input_path)));

    let tokens: Vec<TokenInfo> = iter::from_fn(|| match lexer.next_token() {
        Ok(token) => {
            let cur = lexer.cursor();
            let col = cur.column.saturating_sub(token_len(&token));
            Some(TokenInfo {
                line: cur.line,
                col,
                text: format_token(&token),
            })
        }
        Err(LexerError::EndOfInput) => None,
        Err(e) => panic!("Lexer error: {:?}", e),
    })
    .collect();

    let output: String = input_lines
        .iter()
        .enumerate()
        .filter_map(|(i, _)| {
            let line_tokens: Vec<&TokenInfo> = tokens.iter().filter(|t| t.line == i).collect();
            if line_tokens.is_empty() {
                return None;
            }
            let indent = line_tokens[0].col;
            Some(format!(
                "{}{}",
                " ".repeat(indent),
                line_tokens
                    .iter()
                    .map(|t| t.text.as_str())
                    .collect::<String>()
            ))
        })
        .collect::<Vec<_>>()
        .join("\n");

    fs::write("tests/example/output.txt", output).expect("Failed to write output file");
}

fn token_len(token: &Token) -> usize {
    match token {
        Token::Keyword(s) | Token::Identifier(s) | Token::Operator(s) | Token::Punctuation(s) => {
            s.len()
        }
        Token::Integer(n) => n.to_string().len(),
    }
}

fn format_token(token: &Token) -> String {
    match token {
        Token::Keyword(s) => format!("<Keyword, \"{}\">", s),
        Token::Identifier(s) => format!("<Identifier, \"{}\">", s),
        Token::Integer(n) => format!("<Integer, \"{}\">", n),
        Token::Operator(s) => format!("<Operator, \"{}\">", s),
        Token::Punctuation(s) => format!("<Punctuation, \"{}\">", s),
    }
}
