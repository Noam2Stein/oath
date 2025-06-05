use std::{
    fmt::{self, Formatter},
    sync::RwLock,
};

use derive_new::new;
use string_interner::{
    StringInterner,
    backend::{Backend, BucketBackend},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrId(<BucketBackend as Backend>::Symbol);

#[derive(Debug, new)]
pub struct Interner {
    #[new(value = "RwLock::new(StringInterner::new())")]
    internal: RwLock<StringInterner<BucketBackend>>,
}

impl Interner {
    pub fn intern(&self, str: &str) -> StrId {
        StrId(self.internal.write().unwrap().get_or_intern(str))
    }

    pub fn unintern(&self, str_id: StrId) -> String {
        self.internal.read().unwrap().resolve(str_id.0).unwrap().to_string()
    }

    pub fn unintern_fmt(&self, str_id: StrId, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.internal.read().unwrap().resolve(str_id.0).unwrap())
    }
}

impl Clone for Interner {
    fn clone(&self) -> Self {
        Self {
            internal: RwLock::new(self.internal.read().unwrap().clone()),
        }
    }
}
