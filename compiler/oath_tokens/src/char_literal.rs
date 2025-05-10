use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, new, Spanned)]
#[display("{char:?}")]
pub struct CharLiteral {
    #[span]
    pub span: Span,
    pub char: char,
}

verify_token_type!(CharLiteral);

impl Highlight for CharLiteral {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        h.highlight(self.span, color);
    }
}
