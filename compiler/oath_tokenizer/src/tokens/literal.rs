use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal {
    Char(CharLiteral),
    Float(FloatLiteral),
    Int(IntLiteral),
    Str(StrLiteral),
}

#[allow(private_bounds)]
pub trait LiteralType: TokenType + Copy + TryFrom<Literal> {}

impl LiteralType for Literal {}
impl TokenType for Literal {}
impl Seal for Literal {}

impl TryFrom<TokenTree> for Literal {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for Literal {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(value) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}

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
