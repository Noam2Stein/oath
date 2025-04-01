use oath_src::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HighlightColor {
    Green,
    Cyan,
    Yellow,
}

#[derive(Debug)]
pub struct Highlighter {
    highlights: Vec<(Span, HighlightColor)>,
}

impl Highlighter {
    pub fn highlight(&mut self, item: impl OptionSpanned, color: HighlightColor) {
        let span = match item.option_span() {
            Some(span) => span,
            None => return,
        };

        self.highlights.push((span, color));
    }

    pub fn highlights(&self) -> &[(Span, HighlightColor)] {
        &self.highlights
    }
}

impl Highlighter {
    pub fn new() -> Self {
        Self {
            highlights: Vec::new(),
        }
    }
}
