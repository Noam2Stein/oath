use crate::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    #[inline(always)]
    pub fn from_start_end(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    #[inline(always)]
    pub fn from_start_len(start: Position, len: u32) -> Self {
        Self {
            start,
            end: start + len,
        }
    }
    #[inline(always)]
    pub fn from_end_len(end: Position, len: u32) -> Self {
        Self {
            start: end - len,
            end,
        }
    }

    #[inline(always)]
    pub fn start(self) -> Position {
        self.start
    }
    #[inline(always)]
    pub fn end(self) -> Position {
        self.end
    }

    #[inline(always)]
    pub fn connect(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    #[inline(always)]
    pub fn end_of_file() -> Self {
        Self {
            start: Position::end_of_file(),
            end: Position::end_of_file(),
        }
    }
}
