use crate::{Position, SpanLengthed, SpanLined};

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
    pub fn start(self) -> Position {
        self.start
    }
    #[inline(always)]
    pub fn end(self) -> Position {
        self.end
    }

    #[inline(always)]
    pub fn lined(self) -> Option<SpanLined> {
        if self.start.line == self.end.line {
            Some(SpanLined::from_start_len(
                self.start,
                self.end.char - self.start.char,
            ))
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn len(self) -> Option<u32> {
        self.lined().map(SpanLined::len)
    }
    #[inline(always)]
    pub fn line(self) -> Option<u32> {
        self.lined().map(SpanLined::line)
    }

    #[inline(always)]
    pub fn lengthed<const N: u32>(self) -> Option<SpanLengthed<N>> {
        if self.start.line == self.end.line && self.end.char - self.start.char == N {
            Some(SpanLengthed::from_start(self.start))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn connect(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}
