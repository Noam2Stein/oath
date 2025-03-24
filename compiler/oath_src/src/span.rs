use std::ops::{Add, AddAssign};

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
}

impl Add for Span {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            start: self.start.min(rhs.start),
            end: self.end.max(rhs.end),
        }
    }
}

impl AddAssign for Span {
    fn add_assign(&mut self, rhs: Self) {
        self.start = self.start.min(rhs.start);
        self.end = self.end.max(rhs.end);
    }
}
