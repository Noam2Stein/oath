use std::mem::transmute;

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
