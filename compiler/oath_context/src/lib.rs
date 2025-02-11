use std::{
    fmt::{self, Formatter},
    sync::Mutex,
};

use oath_src::{Span, Spanned};
use string_interner::{
    backend::{Backend, BucketBackend},
    StringInterner,
};

#[derive(Debug)]
pub struct Context {
    pub errors: Vec<Error>,
    pub warnings: Vec<Warning>,
    str_interner: StringInterner<BucketBackend>,
}

#[derive(Debug, Clone, Copy)]
pub struct ContextHandle<'ctx>(pub &'ctx Mutex<Context>);

#[derive(Debug, Clone, Hash)]
pub struct Error {
    span: Span,
    pub message: String,
}

#[derive(Debug, Clone, Hash)]
pub struct Warning {
    span: Span,
    pub message: String,
}

pub type StrId = <BucketBackend as Backend>::Symbol;

impl Context {
    pub fn new() -> Self {
        Context {
            errors: Vec::new(),
            warnings: Vec::new(),
            str_interner: StringInterner::new(),
        }
    }
}

impl<'ctx> ContextHandle<'ctx> {
    pub fn push_error(self, error: impl Into<Error>) {
        self.0.lock().unwrap().errors.push(error.into());
    }
    pub fn push_warning(self, warning: impl Into<Warning>) {
        self.0.lock().unwrap().warnings.push(warning.into());
    }

    pub fn intern(self, str: &str) -> StrId {
        self.0.lock().unwrap().str_interner.get_or_intern(str)
    }
    pub fn unintern(self, str_id: StrId) -> String {
        self.0
            .lock()
            .unwrap()
            .str_interner
            .resolve(str_id)
            .unwrap()
            .to_string()
    }
    pub fn unintern_fmt(self, str_id: StrId, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            self.0.lock().unwrap().str_interner.resolve(str_id).unwrap()
        )
    }
}

impl Spanned for Error {
    fn span(&self) -> Span {
        self.span
    }
}

impl Error {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

impl Spanned for Warning {
    fn span(&self) -> Span {
        self.span
    }
}

impl Warning {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}
