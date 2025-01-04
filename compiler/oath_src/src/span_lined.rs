use crate::{Position, Span, SpanLengthed};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpanLined {
    start: Position,
    end_char: u32,
}

impl SpanLined {
    #[inline(always)]
    pub fn from_start_end(start: Position, end: Position) -> Option<Self> {
        if start.line == end.line {
            Some(Self {
                start,
                end_char: end.char,
            })
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn from_start_len(start: Position, len: u32) -> Self {
        Self {
            start,
            end_char: start.char + len,
        }
    }
    #[inline(always)]
    pub fn from_end_len(end: Position, len: u32) -> Self {
        Self {
            start: end - len,
            end_char: end.char,
        }
    }

    #[inline(always)]
    pub fn start(self) -> Position {
        self.start
    }
    #[inline(always)]
    pub fn end(self) -> Position {
        Position::new(self.start.line, self.end_char)
    }
    #[inline(always)]
    pub fn len(self) -> u32 {
        self.end_char - self.start.char
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
    pub fn lengthed<const N: u32>(self) -> Option<SpanLengthed<N>> {
        if self.len() == N {
            Some(SpanLengthed::from_start(self.start))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn connect(self, other: Self) -> Option<Self> {
        if self.line() == other.line() {
            Some(Self {
                start: self.start.min(other.start),
                end_char: self.end_char.max(other.end_char),
            })
        } else {
            None
        }
    }
}
