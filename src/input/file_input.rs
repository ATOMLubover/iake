use std::{fs, path::PathBuf};

use crate::input::{Cursor, Input};

// FileInput 为简单起见，直接转成 String 处理
pub struct FileInput {
    buf: String,
    index: usize,
    cursor: Cursor,
}

impl FileInput {
    pub fn new(path: PathBuf) -> Self {
        let buf = fs::read_to_string(path).expect("Failed to read file");

        Self {
            buf,
            index: 0,
            cursor: Cursor::default(),
        }
    }
}

impl Input for FileInput {
    fn peek(&self) -> Option<char> {
        self.buf.chars().nth(self.index)
    }

    fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.index += 1;

            match c {
                '\n' => {
                    self.cursor.line += 1;
                    self.cursor.column = 0;
                }
                _ => {
                    self.cursor.column += 1;
                }
            }
        }
    }

    fn is_eof(&self) -> bool {
        self.index >= self.buf.len()
    }

    fn cursor(&self) -> Cursor {
        self.cursor
    }
}
