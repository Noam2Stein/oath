use crate::*;

#[derive(Debug, Clone, Copy, Hash, new, Spanned)]
pub struct IntLiteral {
    #[span]
    pub span: Span,
    pub int: u128,
    pub suffix: Option<Ident>,
}

verify_literal_type!(IntLiteral);

impl From<IntLiteral> for TokenTree {
    fn from(value: IntLiteral) -> Self {
        TokenTree::Literal(Literal::Int(value))
    }
}
impl TryFrom<TokenTree> for IntLiteral {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Int(output)) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for IntLiteral {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Int(value)) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}

impl From<IntLiteral> for Literal {
    fn from(value: IntLiteral) -> Self {
        Literal::Int(value)
    }
}
impl TryFrom<Literal> for IntLiteral {
    type Error = ();

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Int(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
