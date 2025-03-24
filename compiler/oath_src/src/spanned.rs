use crate::Span;

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
