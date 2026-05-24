pub mod file_input;

#[cfg(test)]
pub mod test_input;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

pub trait Input {
    // peek 检查当前游标所在的字符
    fn peek(&self) -> Option<char>;

    // advance 将游标向前移动一个字符
    fn advance(&mut self);

    // is_eof 检查是否已经到达输入的末尾
    fn is_eof(&self) -> bool;

    // cursor 返回当前游标的位置
    fn cursor(&self) -> Cursor;
}
