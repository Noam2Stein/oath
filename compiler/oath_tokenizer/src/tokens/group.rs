use oath_diagnostics::{Desc, Fill};
use oath_src::{Span, Spanned};

use crate::Seal;

use super::{Delimiters, DelimitersType, TokenTree, TokenType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group<D: DelimitersType = Delimiters> {
    pub delimiters: D,
    pub tokens: Vec<TokenTree>,
}

impl TokenType for Group {}
impl Seal for Group {}

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
        D::desc()
    }
}

impl<D: DelimitersType> IntoIterator for Group<D> {
    type IntoIter = <Vec<TokenTree> as IntoIterator>::IntoIter;
    type Item = TokenTree;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}
