use oath_src::Span;

pub trait Fill {
    fn fill(span: Span) -> Self;
}
