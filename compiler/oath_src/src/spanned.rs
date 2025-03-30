use crate::Span;

pub trait Spanned {
    fn span(&self) -> Span;
}

pub trait OptionSpanned {
    fn option_span(&self) -> Option<Span>;
}

impl<T: Spanned> OptionSpanned for T {
    fn option_span(&self) -> Option<Span> {
        Some(self.span())
    }
}
impl<T: OptionSpanned> OptionSpanned for Option<T> {
    fn option_span(&self) -> Option<Span> {
        self.as_ref().map_or(None, T::option_span)
    }
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
