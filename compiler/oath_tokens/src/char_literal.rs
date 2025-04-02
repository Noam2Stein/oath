use crate::*;

#[derive(Debug, Clone, Copy, Hash, Display, new, Spanned)]
#[display("{char:?}")]
pub struct CharLiteral {
    #[span]
    pub span: Span,
    pub char: char,
}

verify_literal_type!(CharLiteral);

impl From<CharLiteral> for TokenTree {
    fn from(value: CharLiteral) -> Self {
        TokenTree::Literal(Literal::Char(value))
    }
}
impl TryFrom<TokenTree> for CharLiteral {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Char(value)) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for CharLiteral {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Char(value)) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}

impl From<CharLiteral> for Literal {
    fn from(value: CharLiteral) -> Self {
        Literal::Char(value)
    }
}
impl TryFrom<Literal> for CharLiteral {
    type Error = ();

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Char(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
