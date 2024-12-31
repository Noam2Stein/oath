use oath_src::{Span, Spanned};

use super::{LiteralType, LiteralTypeSeal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntLiteral {
    int: u128,
    span: Span,
}

impl LiteralType for IntLiteral {}
impl LiteralTypeSeal for IntLiteral {}
impl Spanned for IntLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

impl IntLiteral {
    #[inline(always)]
    pub fn new(int: u128, span: Span) -> Self {
        Self { int, span }
    }

    #[inline(always)]
    pub fn int(self) -> u128 {
        self.int
    }
}
