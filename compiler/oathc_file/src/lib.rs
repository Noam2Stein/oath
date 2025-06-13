use std::sync::RwLock;

use derive_new::new;
use string_interner::{
    StringInterner,
    backend::{Backend, BucketBackend},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(<BucketBackend as Backend>::Symbol);

#[derive(Debug, new)]
pub struct FileInterner {
    #[new(value = "RwLock::new(StringInterner::new())")]
    internal: RwLock<StringInterner<BucketBackend>>,
}
