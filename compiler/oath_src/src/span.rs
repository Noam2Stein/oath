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

impl Add<Option<Span>> for Span {
    type Output = Self;

    fn add(self, rhs: Option<Span>) -> Self::Output {
        match rhs {
            Some(rhs) => self + rhs,
            None => self,
        }
    }
}
impl AddAssign<Option<Span>> for Span {
    fn add_assign(&mut self, rhs: Option<Span>) {
        if let Some(rhs) = rhs {
            *self += rhs;
        }
    }
}

impl Add<Span> for Option<Span> {
    type Output = Span;

    fn add(self, rhs: Span) -> Self::Output {
        rhs + self
    }
}
impl AddAssign<Span> for Option<Span> {
    fn add_assign(&mut self, rhs: Span) {
        if let Some(lhs) = self {
            *lhs += rhs;
        }
    }
}
