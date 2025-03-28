use std::fmt::{self, Formatter};

use string_interner::{
    backend::{Backend, BucketBackend},
    StringInterner,
};

use crate::ContextHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrId(<BucketBackend as Backend>::Symbol);

#[derive(Debug)]
pub(super) struct Interner {
    str_interner: StringInterner<BucketBackend>,
}

impl<'ctx> ContextHandle<'ctx> {
    pub fn intern(self, str: &str) -> StrId {
        StrId(
            self.0
                .lock()
                .unwrap()
                .interner
                .str_interner
                .get_or_intern(str),
        )
    }
    pub fn unintern(self, str_id: StrId) -> String {
        self.0
            .lock()
            .unwrap()
            .interner
            .str_interner
            .resolve(str_id.0)
            .unwrap()
            .to_string()
    }
    pub fn unintern_fmt(self, str_id: StrId, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            self.0
                .lock()
                .unwrap()
                .interner
                .str_interner
                .resolve(str_id.0)
                .unwrap()
        )
    }
}

impl Interner {
    pub fn new() -> Self {
        Self {
            str_interner: StringInterner::new(),
        }
    }
}
