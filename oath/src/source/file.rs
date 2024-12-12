use std::{
    fmt::{self, Display, Formatter},
    ops::{Index, Range},
};

use crate::tokenization::Tokenizer;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SrcFileId {
    value: u32,
}
impl Display for SrcFileId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}
impl SrcFileId {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct SrcFile {
    id: SrcFileId,
    str: String,
}
impl Spanned for SrcFile {
    fn span(&self) -> Span {
        Span {
            file_id: self.id,
            start: 0,
            len: self.str.len(),
        }
    }
}
impl Index<Span> for SrcFile {
    type Output = str;
    fn index(&self, index: Span) -> &Self::Output {
        &self.str[index.start..index.start + index.len]
    }
}
impl SrcFile {
    pub fn new(id: SrcFileId, str: &str) -> Self {
        Self {
            id,
            str: str.replace("\r\n", "\n").replace("\n\r", "\n"),
        }
    }

    pub fn id(&self) -> SrcFileId {
        self.id
    }
    pub fn str(&self) -> &str {
        &self.str
    }

    #[allow(unused_must_use)]
    pub fn span_from_range(&self, value: Range<usize>) -> Span {
        &self.str[value.clone()];
        Span {
            file_id: self.id,
            start: value.start,
            len: value.end - value.start,
        }
    }
    pub fn end_span(&self) -> Span {
        self.span_from_range(self.str.len()..self.str.len())
    }

    pub fn line_number(&self, position: usize) -> usize {
        self.str()
            .char_indices()
            .filter(|(_, c)| *c == '\n')
            .map(|(index, _)| index)
            .filter(|index| *index < position)
            .count()
            + 1
    }

    pub fn tokenize(&self) -> Tokenizer {
        Tokenizer::new(self)
    }
}
