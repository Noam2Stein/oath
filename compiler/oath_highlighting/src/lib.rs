use oath_src::*;

mod highlight;
pub use highlight::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HighlightColor {
    Green,
    Blue,
    Cyan,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HighlightInfo {
    pub span: Span,
    pub color: HighlightColor,
}

#[derive(Debug, Clone)]
pub struct Highlighter {
    pub highlights: Vec<HighlightInfo>,
}

impl Highlighter {
    pub fn new() -> Self {
        Self { highlights: Vec::new() }
    }

    pub fn highlight(&mut self, span: Span, color: HighlightColor) {
        self.highlights.push(HighlightInfo { span, color });
    }
}
