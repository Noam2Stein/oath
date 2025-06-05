use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    pub const ZERO: Self = Self::from_start_end(Position::ZERO, Position::ZERO);

    pub const fn from_start_end(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    pub fn from_start_len(start: Position, len: u32) -> Self {
        Self { start, end: start + len }
    }
    pub fn from_end_len(end: Position, len: u32) -> Self {
        Self { start: end - len, end }
    }

    pub fn start(self) -> Position {
        self.start
    }
    pub fn end(self) -> Position {
        self.end
    }

    pub fn line(self) -> Option<u32> {
        if self.start.line == self.end.line {
            Some(self.start.line)
        } else {
            None
        }
    }
    pub fn len(self) -> Option<u32> {
        if self.start.line == self.end.line {
            Some(self.end.char - self.start.char)
        } else {
            None
        }
    }
}
