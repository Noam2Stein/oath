use std::fmt::{self, Formatter};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, new, Spanned)]
pub struct FloatLiteral {
    #[span]
    pub span: Span,
    pub integral: u128,
    pub leading_zeros: u128,
    pub fractional: u128,
    pub suffix: Option<Ident>,
}

verify_literal_type!(FloatLiteral);

impl From<FloatLiteral> for TokenTree {
    fn from(value: FloatLiteral) -> Self {
        TokenTree::Literal(Literal::Float(value))
    }
}
impl TryFrom<TokenTree> for FloatLiteral {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Float(value)) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for FloatLiteral {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Float(value)) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}

impl From<FloatLiteral> for Literal {
    fn from(value: FloatLiteral) -> Self {
        Literal::Float(value)
    }
}
impl TryFrom<Literal> for FloatLiteral {
    type Error = ();

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Float(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl InternedDisplay for FloatLiteral {
    fn interned_fmt(&self, f: &mut Formatter, interner: &Interner) -> fmt::Result {
        write!(f, "{}", self.integral,)?;

        write!(f, ".{}", "0".repeat(self.leading_zeros as usize),)?;

        write!(f, "{}", self.fractional,)?;

        if let Some(suffix) = self.suffix {
            write!(f, "{}", Interned(&suffix, interner))?;
        };

        Ok(())
    }
}
