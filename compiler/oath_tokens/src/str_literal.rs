use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, new, Spanned, InternedDisplay)]
#[display("\"{}\"", self.str_id)]
pub struct StrLiteral {
    #[span]
    pub span: Span,
    pub str_id: StrId,
}

verify_token_type!(StrLiteral);

impl Highlight for StrLiteral {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        h.highlight(self.span, color);
    }
}
