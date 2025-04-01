use crate::*;

#[derive(Debug, Clone, Copy, Hash, new, Spanned)]
pub struct StrLiteral {
    #[span]
    pub span: Span,
    pub str_id: StrId,
}

verify_literal_type!(StrLiteral);

impl From<StrLiteral> for TokenTree {
    fn from(value: StrLiteral) -> Self {
        TokenTree::Literal(Literal::Str(value))
    }
}
impl TryFrom<TokenTree> for StrLiteral {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Str(value)) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for StrLiteral {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Str(value)) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}

impl From<StrLiteral> for Literal {
    fn from(value: StrLiteral) -> Self {
        Literal::Str(value)
    }
}
impl TryFrom<Literal> for StrLiteral {
    type Error = ();

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Str(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
