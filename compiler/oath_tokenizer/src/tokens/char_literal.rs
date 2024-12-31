use oath_src::{Span, Spanned};

use super::{LiteralType, LiteralTypeSeal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharLiteral {
    char: char,
    span: Span,
}

impl LiteralType for CharLiteral {}
impl LiteralTypeSeal for CharLiteral {}
impl Spanned for CharLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

impl CharLiteral {
    #[inline(always)]
    pub fn new(char: char, span: Span) -> Self {
        Self { char, span }
    }

    #[inline(always)]
    pub fn char(self) -> char {
        self.char
    }
}
