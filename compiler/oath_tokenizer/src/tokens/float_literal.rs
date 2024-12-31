use oath_src::{Span, Spanned};

use super::{LiteralType, LiteralTypeSeal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FloatLiteral {
    integral: u128,
    leading_zeros: u128,
    fractional: u128,
    span: Span,
}

impl LiteralType for FloatLiteral {}
impl LiteralTypeSeal for FloatLiteral {}
impl Spanned for FloatLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

impl FloatLiteral {
    #[inline(always)]
    pub fn new(integral: u128, leading_zeros: u128, fractional: u128, span: Span) -> Self {
        Self {
            integral,
            leading_zeros,
            fractional,
            span,
        }
    }

    #[inline(always)]
    pub fn integral(self) -> u128 {
        self.integral
    }
    #[inline(always)]
    pub fn leading_zeros(self) -> u128 {
        self.leading_zeros
    }
    #[inline(always)]
    pub fn fractional(self) -> u128 {
        self.fractional
    }
}
