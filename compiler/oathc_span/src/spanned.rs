use nonempty::NonEmpty;

use super::*;

pub use oathc_span_proc_macros::Spanned;

pub trait Spanned: OptionSpanned {
    fn span(&self) -> Span;
}

impl Spanned for Span {
    fn span(&self) -> Span {
        *self
    }
}

impl<T: Spanned> Spanned for &T {
    fn span(&self) -> Span {
        T::span(&self)
    }
}

impl<T: Spanned> Spanned for Box<T> {
    fn span(&self) -> Span {
        T::span(&self)
    }
}

impl<T: Spanned> Spanned for NonEmpty<T> {
    fn span(&self) -> Span {
        self.iter().fold(None, |a, b| Some(a.connect(b.span()))).unwrap()
    }
}
