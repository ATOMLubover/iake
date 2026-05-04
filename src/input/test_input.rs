use crate::input::{Input, Position};

pub struct TestInput {
    chars: Vec<char>,
    idx: usize,
}

impl TestInput {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            idx: 0,
        }
    }
}

impl Input for TestInput {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.idx).copied()
    }

    fn advance(&mut self) {
        if self.idx < self.chars.len() {
            self.idx += 1;
        }
    }

    fn is_eof(&self) -> bool {
        self.idx >= self.chars.len()
    }

    fn current_position(&self) -> Position {
        Position::default()
    }
}
