use std::{fmt::Debug, hash::Hash};

use oath_diagnostics::{Desc, Fill};
use oath_src::{Span, Spanned};
use oath_tokenizer_macros::TokenDowncast;

use crate::Seal;

use super::{CharLiteral, FloatLiteral, IntLiteral, StrLiteral, TokenDowncastFrom, TokenType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TokenDowncast)]
pub enum Literal {
    Char(CharLiteral),
    Float(FloatLiteral),
    Int(IntLiteral),
    Str(StrLiteral),
}

#[allow(private_bounds)]
pub trait LiteralType: TokenType + TokenDowncastFrom<Literal> {}

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
