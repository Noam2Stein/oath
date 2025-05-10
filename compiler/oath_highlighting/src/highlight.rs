use super::*;

pub trait Highlight {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter);
}

impl<T: Highlight> Highlight for Option<T> {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        if let Some(value) = self {
            value.highlight(color, h);
        }
    }
}

impl<T: Highlight> Highlight for Box<T> {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        T::highlight(&self, color, h);
    }
}
