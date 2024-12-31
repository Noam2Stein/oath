#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl Span {
    #[inline(always)]
    pub fn from_start_end(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    #[inline(always)]
    pub fn from_start_len(start: usize, len: usize) -> Self {
        Self {
            start,
            end: start + len,
        }
    }
    #[inline(always)]
    pub fn from_end_len(end: usize, len: usize) -> Self {
        Self {
            start: end - len,
            end,
        }
    }

    #[inline(always)]
    pub fn start(self) -> usize {
        self.start
    }
    #[inline(always)]
    pub fn end(self) -> usize {
        self.end
    }
    #[inline(always)]
    pub fn len(self) -> usize {
        self.end - self.start
    }

    #[inline(always)]
    pub fn connect(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}
