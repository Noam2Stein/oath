use std::{fmt::Debug, hash::Hash};

use oath_src::{Span, Spanned};

use super::{CharLiteral, FloatLiteral, IntLiteral, StringLiteral};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal {
    Char(CharLiteral),
    Float(FloatLiteral),
    Int(IntLiteral),
    String(StringLiteral),
}

#[allow(private_bounds)]
pub trait LiteralType:
    LiteralTypeSeal + Send + Sync + Debug + Clone + Eq + Ord + Hash + Spanned
{
}
pub(crate) trait LiteralTypeSeal {}

impl LiteralType for Literal {}
impl LiteralTypeSeal for Literal {}
impl Spanned for Literal {
    fn span(&self) -> Span {
        match self {
            Self::Char(lit) => lit.span(),
            Self::Float(lit) => lit.span(),
            Self::Int(lit) => lit.span(),
            Self::String(lit) => lit.span(),
        }
    }
}
