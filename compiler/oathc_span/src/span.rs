use std::cmp::Ordering;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    file: FileId,
    start_line: u32,
    start_char: u32,
    end_line: u32,
    end_char: u32,
}

impl Span {
    pub fn from_range(file: FileId, start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> Self {
        Self {
            file,
            start_line,
            start_char,
            end_line,
            end_char,
        }
    }
    pub fn from_start(start: Position, len: u32) -> Self {
        Self {
            file: start.file,
            start_line: start.line,
            start_char: start.char,
            end_line: start.line,
            end_char: start.char + len,
        }
    }
    pub fn from_end(end: Position, len: u32) -> Self {
        Self {
            file: end.file,
            start_line: end.line,
            start_char: end.char - len,
            end_line: end.line,
            end_char: end.char,
        }
    }

    pub fn from_positions(start: Position, end: Position) -> Option<Self> {
        if start.file == end.file {
            Some(Self {
                file: start.file,
                start_line: start.line,
                start_char: start.char,
                end_line: end.line,
                end_char: end.char,
            })
        } else {
            None
        }
    }

    pub fn file(self) -> FileId {
        self.file
    }
    pub fn start(self) -> Position {
        Position::new(self.file, self.start_line, self.start_char)
    }
    pub fn end(self) -> Position {
        Position::new(self.file, self.end_line, self.end_char)
    }

    pub fn line(self) -> Option<u32> {
        if self.start_line == self.end_line {
            Some(self.start_line)
        } else {
            None
        }
    }
    pub fn len(self) -> Option<u32> {
        if self.start_line == self.end_line {
            Some(self.end_char - self.start_char)
        } else {
            None
        }
    }

    pub fn index_src(self, src: &str) -> &str {
        let lines = src.lines();

        let all_lines: Vec<&str> = lines.collect();

        let start_line = self.start_line as usize;
        let end_line = self.end_line as usize;
        if start_line >= all_lines.len() || end_line >= all_lines.len() || start_line > end_line {
            return "";
        }

        let start_str = all_lines[start_line];
        let end_str = all_lines[end_line];

        let start_byte_offset = start_str.char_indices().nth(self.start_char as usize).map(|(i, _)| i);
        let end_byte_offset = end_str.char_indices().nth(self.end_char as usize).map(|(i, _)| i);

        let start_byte = match start_byte_offset {
            Some(i) => {
                let prefix_len: usize = all_lines[..start_line].iter().map(|l| l.len() + 1).sum();
                prefix_len + i
            }
            None => return "",
        };

        let end_byte = match end_byte_offset {
            Some(i) => {
                let prefix_len: usize = all_lines[..end_line].iter().map(|l| l.len() + 1).sum();
                prefix_len + i
            }
            None => return "",
        };

        if start_byte > end_byte || end_byte > src.len() {
            return "";
        }

        &src[start_byte..end_byte]
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start().partial_cmp(&other.start())
    }
}
impl Ord for Span {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start().cmp(&other.start())
    }
}
