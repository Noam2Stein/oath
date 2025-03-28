use std::mem::replace;

use oath_src::Span;

use crate::ContextHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HighlightColor {
    Green,
    Cyan,
    Yellow,
}

pub trait Highlightable {
    fn highlight_span(&self) -> Option<Span>;
}

#[derive(Debug)]
pub(super) struct Highlighter {
    highlights: Vec<(Span, HighlightColor)>,
}

impl<'ctx> ContextHandle<'ctx> {
    pub fn highlight(self, item: impl Highlightable, color: HighlightColor) {
        let span = match item.highlight_span() {
            Some(span) => span,
            None => return,
        };

        self.0
            .lock()
            .unwrap()
            .highlighter
            .highlights
            .push((span, color));
    }

    pub fn collect_highlights(self) -> Vec<(Span, HighlightColor)> {
        replace(
            &mut self.0.lock().unwrap().highlighter.highlights,
            Vec::new(),
        )
    }
}

impl Highlighter {
    pub fn new() -> Self {
        Self {
            highlights: Vec::new(),
        }
    }
}

impl<'a, T: Highlightable> Highlightable for &'a T {
    fn highlight_span(&self) -> Option<Span> {
        T::highlight_span(self)
    }
}
impl<T: Highlightable> Highlightable for Option<T> {
    fn highlight_span(&self) -> Option<Span> {
        self.as_ref().map_or(None, T::highlight_span)
    }
}

impl Highlightable for Span {
    fn highlight_span(&self) -> Option<Span> {
        Some(*self)
    }
}
