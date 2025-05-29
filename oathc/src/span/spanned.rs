use nonempty::NonEmpty;

use super::*;

pub use oathc_proc_macros::Spanned;

pub trait Spanned {
    fn span(&self) -> Span;
}

impl Spanned for Span {
    fn span(&self) -> Span {
        *self
    }
}

impl<T: Spanned> Spanned for Box<T> {
    fn span(&self) -> Span {
        T::span(&self)
    }
}

impl<T: Spanned> Spanned for NonEmpty<T> {
    fn span(&self) -> Span {
        self.iter().fold(None, |a, b| Span::connect(a, b.span())).unwrap()
    }
}
