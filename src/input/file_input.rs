use std::{fs, path::PathBuf};

use crate::input::{Cursor, Input};

// FileInput 为简单起见，直接转成 String 处理
pub struct FileInput {
    chars: Vec<char>,
    index: usize,
    cursor: Cursor,
}

impl FileInput {
    pub fn new(path: PathBuf) -> Self {
        let chars = fs::read_to_string(path)
            .expect("Failed to read file")
            .chars()
            .collect();

        Self {
            chars,
            index: 0,
            cursor: Cursor::default(),
        }
    }
}

impl Input for FileInput {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
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
        self.index >= self.chars.len()
    }

    fn cursor(&self) -> Cursor {
        self.cursor
    }
}
