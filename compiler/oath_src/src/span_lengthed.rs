use crate::{Position, Span, SpanLined};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpanLengthed<const N: u32> {
    start: Position,
}

impl<const N: u32> SpanLengthed<N> {
    #[inline(always)]
    pub fn from_start(start: Position) -> Self {
        Self { start }
    }
    #[inline(always)]
    pub fn from_end(end: Position) -> Self {
        Self { start: end - N }
    }

    #[inline(always)]
    pub fn start(self) -> Position {
        self.start
    }
    #[inline(always)]
    pub fn end(self) -> Position {
        self.start + N
    }
    #[inline(always)]
    pub fn len(self) -> u32 {
        N
    }
    #[inline(always)]
    pub fn line(self) -> u32 {
        self.start.line
    }

    #[inline(always)]
    pub fn unlined(self) -> Span {
        Span::from_start_end(self.start, self.end())
    }

    #[inline(always)]
    pub fn unlengthed(self) -> SpanLined {
        SpanLined::from_start_len(self.start, N)
    }
}
