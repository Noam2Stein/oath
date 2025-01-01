use oath_src::{Span, Spanned};

use crate::Seal;

use super::LiteralType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringLiteral {
    str: String,
    span: Span,
}

impl LiteralType for StringLiteral {}
impl Seal for StringLiteral {}
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
