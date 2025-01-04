use crate::Span;

pub trait Spanned {
    fn span(&self) -> Span;
}
