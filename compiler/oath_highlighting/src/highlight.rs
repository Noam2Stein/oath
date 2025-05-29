use super::*;

pub trait Highlight {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter);
}

impl<T: Highlight> Highlight for &T {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        T::highlight(&self, color, h);
    }
}
impl<T: Highlight> Highlight for Option<T> {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        if let Some(value) = self {
            value.highlight(color, h);
        }
    }
}
impl<T: Highlight> Highlight for Try<T> {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        if let Self::Success(value) = self {
            value.highlight(color, h);
        }
    }
}
impl<T: Highlight> Highlight for Box<T> {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        T::highlight(&self, color, h);
    }
}

impl Highlight for Ident {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        h.highlight_span(self.span, color);
    }
}

impl Highlight for Keyword {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        h.highlight_span(self.span, color);
    }
}
with_tokens!($(
    impl Highlight for $keyword_type {
        fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
            h.highlight_span(self.0, color);
        }
    }
)*);

impl Highlight for IntLiteral {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        h.highlight_span(self.span, color);
    }
}
impl Highlight for FloatLiteral {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        h.highlight_span(self.span, color);
    }
}
