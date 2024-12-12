use std::fmt::{self, Display, Formatter};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error {
    span: Span,
    str: String,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.str.fmt(f)
    }
}
impl Spanned for Error {
    fn span(&self) -> Span {
        self.span
    }
}
impl Error {
    pub fn new(span: Span, str: impl Into<String>) -> Self {
        Self {
            span,
            str: str.into(),
        }
    }

    pub fn str(&self) -> &str {
        &self.str
    }
}

#[derive(Debug, Hash)]
pub struct ErrorsHandle<'errs> {
    errors: &'errs mut Vec<Error>,
}
impl<'errs> ErrorsHandle<'errs> {
    pub fn new(errors: &'errs mut Vec<Error>) -> Self {
        Self { errors }
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    pub fn push(&mut self, error: Error) {
        self.errors.push(error);
    }
}
