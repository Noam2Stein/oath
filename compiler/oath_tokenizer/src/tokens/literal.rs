use std::{fmt::Debug, hash::Hash};

use oath_diagnostics::{Desc, Fill};
use oath_src::{Span, Spanned};

use crate::Seal;

use super::{CharLiteral, FloatLiteral, IntLiteral, StrLiteral, TokenTree, TokenType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal {
    Char(CharLiteral),
    Float(FloatLiteral),
    Int(IntLiteral),
    Str(StrLiteral),
}

#[allow(private_bounds)]
pub trait LiteralType:
    TokenType + Send + Sync + Debug + Clone + Eq + Ord + Hash + Spanned + TryFrom<Literal>
where
    for<'a> &'a Self: TryFrom<&'a Literal>,
    for<'a> &'a Self: TryFrom<&'a TokenTree>,
{
}

impl LiteralType for Literal {}
impl TokenType for Literal {}
impl Seal for Literal {}
impl Spanned for Literal {
    fn span(&self) -> Span {
        match self {
            Self::Char(lit) => lit.span(),
            Self::Float(lit) => lit.span(),
            Self::Int(lit) => lit.span(),
            Self::Str(lit) => lit.span(),
        }
    }
}
impl Fill for Literal {
    fn fill(span: Span) -> Self {
        Self::Int(IntLiteral::fill(span))
    }
}
impl Desc for Literal {
    fn desc() -> &'static str {
        "a literal"
    }
}
impl TryFrom<TokenTree> for Literal {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(output) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for &'a Literal {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(output) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
