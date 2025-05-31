use super::diagnostics::*;
use super::span::*;
use super::tokens::*;

mod highlight;
pub(super) use highlight::*;

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
