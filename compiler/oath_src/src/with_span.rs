use crate::{Span, Spanned};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithSpan<T> {
    pub inner: T,
    span: Span,
}

impl<T> WithSpan<T> {
    pub fn new(inner: T, span: Span) -> Self {
        Self { inner, span }
    }
}
impl<T> Spanned for WithSpan<T> {
    fn span(&self) -> Span {
        self.span
    }
}
