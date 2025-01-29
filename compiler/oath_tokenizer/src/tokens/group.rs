use oath_diagnostics::{Desc, Fill};
use oath_src::{Span, Spanned};

use crate::Seal;

use super::{Delimiters, DelimitersType, TokenTree, TokenType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group<D: DelimitersType = Delimiters> {
    pub delimiters: D,
    pub tokens: Vec<TokenTree>,
}

impl<D: DelimitersType> TokenType for Group<D> {}
impl<D: DelimitersType> Seal for Group<D> {}
impl<D: DelimitersType> Spanned for Group<D> {
    fn span(&self) -> Span {
        self.delimiters.span()
    }
}
impl<D: DelimitersType> Fill for Group<D> {
    fn fill(span: Span) -> Self {
        Self {
            delimiters: D::fill(span),
            tokens: Vec::default(),
        }
    }
}
impl<D: DelimitersType> Desc for Group<D> {
    fn desc() -> &'static str {
        D::group_desc()
    }
}
impl<D: DelimitersType> TryFrom<TokenTree> for Group<D> {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Group(output) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for &'a Group {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Float(output)) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
