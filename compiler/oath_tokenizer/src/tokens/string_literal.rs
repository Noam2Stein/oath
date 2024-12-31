use oath_src::{Span, Spanned};

use super::{LiteralType, LiteralTypeSeal};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringLiteral {
    str: String,
    span: Span,
}

impl LiteralType for StringLiteral {}
impl LiteralTypeSeal for StringLiteral {}
impl Spanned for StringLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

impl StringLiteral {
    #[inline(always)]
    pub fn new(str: String, span: Span) -> Self {
        Self { str, span }
    }

    #[inline(always)]
    pub fn str(&self) -> &str {
        &self.str
    }
}
