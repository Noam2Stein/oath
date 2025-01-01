use std::{fmt::Debug, hash::Hash};

use oath_src::{Span, Spanned};

use crate::Seal;

use super::{CharLiteral, FloatLiteral, IntLiteral, StrLiteral};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal {
    Char(CharLiteral),
    Float(FloatLiteral),
    Int(IntLiteral),
    Str(StrLiteral),
}

#[allow(private_bounds)]
pub trait LiteralType: Seal + Send + Sync + Debug + Clone + Eq + Ord + Hash + Spanned {}

impl LiteralType for Literal {}
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
