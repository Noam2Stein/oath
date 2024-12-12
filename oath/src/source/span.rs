use std::ops::Range;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub(super) file_id: SrcFileId,
    pub(super) start: usize,
    pub(super) len: usize,
}
impl Span {
    pub fn file_id(self) -> SrcFileId {
        self.file_id
    }
    pub fn start(self) -> usize {
        self.start
    }
    pub fn len(self) -> usize {
        self.len
    }
    pub fn end(self) -> usize {
        self.start + self.len
    }
    pub fn range(self) -> Range<usize> {
        self.start()..self.end()
    }

    pub fn join(self, other: Self) -> Self {
        assert_eq!(self.file_id, other.file_id);

        let start = self.start.min(other.start);
        let len = (self.start + self.len).max(other.start + other.len) - start;
        Self {
            file_id: self.file_id,
            start,
            len,
        }
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}
