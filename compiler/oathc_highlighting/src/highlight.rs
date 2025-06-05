use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HighlightColor {
    Green,
    Blue,
    Cyan,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Highlight {
    pub span: Span,
    pub color: HighlightColor,
}
