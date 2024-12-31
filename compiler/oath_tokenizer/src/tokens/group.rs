use oath_src::{Span, Spanned};

use super::{Delimiters, DelimitersType, TokenTree};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group<D: DelimitersType = Delimiters> {
    pub delimiters: D,
    pub tokens: Vec<TokenTree>,
}
impl Spanned for Group {
    fn span(&self) -> Span {
        self.delimiters.span()
    }
}
