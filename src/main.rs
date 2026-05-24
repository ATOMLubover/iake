use std::fs;
use std::path::PathBuf;

use iake::input::file_input::FileInput;
use iake::lexer::result::Error as LexerError;
use iake::parser::Parser;
use iake::parser::result::Error as ParserError;

fn main() {
    for i in 1..=5 {
        let input_path = format!("tests/example/input-{}.txt", i);
        let output_path = format!("tests/example/output-{}.txt", i);
        process_file(&input_path, &output_path);
    }
}

fn process_file(input_path: &str, output_path: &str) {
    let mut parser = Parser::new(FileInput::new(PathBuf::from(input_path)));

    let output = match parser.parse_program() {
        Ok(program) => program.preorder_string(),
        Err(err) => format_parser_error(&err),
    };

    fs::write(output_path, output).expect("Failed to write output file");
}

fn format_parser_error(err: &ParserError) -> String {
    match err {
        ParserError::Lex { err, cursor } => format!(
            "(line {}, col {}): {}",
            cursor.line,
            cursor.column,
            format_lexer_error(err)
        ),
        ParserError::UnexpectedToken {
            expected,
            found,
            cursor,
        } => format!(
            "(line {}, col {}): expected {}, found {}",
            cursor.line, cursor.column, expected, found
        ),
        ParserError::UnexpectedEndOfInput { expected, cursor } => format!(
            "(line {}, col {}): expected {}, found EOF",
            cursor.line, cursor.column, expected
        ),
    }
}

fn format_lexer_error(err: &LexerError) -> String {
    match err {
        LexerError::UnexpectedChar(c) => format!("UnexpectedChar('{}')", c),
        LexerError::InvalidChar(c) => format!("InvalidChar('{}')", c),
        LexerError::InvalidInteger(s) => format!("InvalidInteger(\"{}\")", s),
        LexerError::TokenTooLong(s) => format!("TokenTooLong(\"{}\")", s),
    }
}
