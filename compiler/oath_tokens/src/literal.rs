use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Spanned, InternedDisplay)]
pub enum Literal {
    Char(CharLiteral),
    Float(FloatLiteral),
    Int(IntLiteral),
    Str(StrLiteral),
}

verify_token_type!(Literal);

impl Highlight for Literal {
    fn highlight(&self, color: HighlightColor, h: &mut Highlighter) {
        h.highlight(self.span(), color);
    }
}
