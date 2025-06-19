use super::*;

pub use oathc_highlighting_proc_macros::Highlightable;

pub trait Highlightable {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>);
}

impl Highlightable for Span {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        h.push(Highlight { span: *self, color });
    }
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

impl<T: Highlightable> Highlightable for Vec<T> {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        for item in self {
            item.highlight(color, h);
        }
    }
}
