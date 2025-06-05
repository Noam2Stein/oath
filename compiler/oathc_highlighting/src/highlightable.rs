use super::*;

pub trait Highlightable {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>);
}

impl<T: Highlightable> Highlightable for &T {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        T::highlight(&self, color, h);
    }
}
impl<T: Highlightable> Highlightable for Option<T> {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        if let Some(value) = self {
            value.highlight(color, h);
        }
    }
}
impl<T: Highlightable> Highlightable for Box<T> {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        T::highlight(&self, color, h);
    }
}

/*

impl<T: Highlight> Highlight for Try<T> {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<HighlightInfo>) {
        if let Self::Success(value) = self {
            value.highlight(color, h);
        }
    }
}
impl Highlight for Ident {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<HighlightInfo>) {
        h.push(HighlightInfo { span: self.span, color });
    }
}

impl Highlight for Keyword {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<HighlightInfo>) {
        h.push(HighlightInfo { span: self.span, color });
    }
}
with_tokens!($(
    impl Highlight for $keyword_type {
        fn highlight(&self, color: HighlightColor, h: &mut Vec<HighlightInfo>) {
            h.push(HighlightInfo { span: self.0, color });
        }
    }
)*);

impl Highlight for IntLiteral {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<HighlightInfo>) {
        h.push(HighlightInfo { span: self.span, color });
    }
}
impl Highlight for FloatLiteral {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<HighlightInfo>) {
        h.push(HighlightInfo { span: self.span, color });
    }
}


*/
