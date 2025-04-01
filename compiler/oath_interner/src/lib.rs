use std::fmt::{self, Formatter};

use string_interner::{
    StringInterner,
    backend::{Backend, BucketBackend},
};

mod interned_display;
pub use interned_display::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrId(<BucketBackend as Backend>::Symbol);

#[derive(Debug, Clone)]
pub struct Interner {
    internal: StringInterner<BucketBackend>,
}

impl Interner {
    pub fn intern(&mut self, str: &str) -> StrId {
        StrId(self.internal.get_or_intern(str))
    }
    pub fn unintern(self, str_id: StrId) -> String {
        self.internal.resolve(str_id.0).unwrap().to_string()
    }
    pub fn unintern_fmt(self, str_id: StrId, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.internal.resolve(str_id.0).unwrap())
    }
}

impl Interner {
    pub fn new() -> Self {
        Self {
            internal: StringInterner::new(),
        }
    }
}
