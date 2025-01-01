use std::{mem::transmute, ops::Index};

use crate::Span;

#[repr(transparent)]
pub struct SrcFile(str);

impl SrcFile {
    #[inline(always)]
    pub fn from_str(str: &str) -> &Self {
        unsafe { transmute(str) }
    }
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        unsafe { transmute(self) }
    }
}

impl Index<Span> for SrcFile {
    type Output = str;
    fn index(&self, index: Span) -> &Self::Output {
        &self.as_str()[index.start()..index.end()]
    }
}
