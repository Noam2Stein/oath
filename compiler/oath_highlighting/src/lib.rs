use oath_src::*;

mod highlight;
pub use highlight::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HighlightColor {
    Blue,
    Green,
    Cyan,
    Yellow,
}

#[derive(Debug, Clone)]
pub struct Highlighter {
    highlights: Vec<(Span, HighlightColor)>,
}

impl Highlighter {
    pub fn highlight(&mut self, span: Span, color: HighlightColor) {
        self.highlights.push((span, color));
    }

    pub fn highlights(&self) -> &[(Span, HighlightColor)] {
        &self.highlights
    }
}

impl Highlighter {
    pub fn new() -> Self {
        Self { highlights: Vec::new() }
    }
}
