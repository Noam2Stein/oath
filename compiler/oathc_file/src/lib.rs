use std::{
    path::{Path, PathBuf},
    sync::RwLock,
};

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

impl FileInterner {
    pub fn intern(&self, file: impl AsRef<Path>) -> FileId {
        FileId(
            self.internal
                .write()
                .unwrap()
                .get_or_intern(file.as_ref().as_os_str().to_str().expect("tried to intern a non utf8 path")),
        )
    }

    pub fn unintern(&self, file_id: FileId) -> PathBuf {
        self.internal.read().unwrap().resolve(file_id.0).unwrap().to_string().into()
    }
}

impl Clone for FileInterner {
    fn clone(&self) -> Self {
        Self {
            internal: RwLock::new(self.internal.read().unwrap().clone()),
        }
    }
}
