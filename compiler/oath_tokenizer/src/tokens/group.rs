use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group<D: DelimitersType = Delimiters> {
    pub delimiters: D,
    pub tokens: Vec<TokenTree>,
}

impl<D: DelimitersType> TokenType for Group<D> {}
impl<D: DelimitersType> Seal for Group<D> {}

impl<D: DelimitersType> TryFrom<TokenTree> for Group<D> {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Group(Group { delimiters, tokens }) = value {
            if let Ok(delimiters) = delimiters.try_into() {
                Ok(Self { delimiters, tokens })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
impl<'a, D: DelimitersType> TryFrom<&'a TokenTree> for Group<D> {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Group(Group { delimiters, tokens }) = value {
            if let Ok(delimiters) = (*delimiters).try_into() {
                Ok(Self {
                    delimiters,
                    tokens: tokens.clone(),
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl<D: DelimitersType> Spanned for Group<D> {
    fn span(&self) -> Span {
        self.delimiters.span()
    }
}

impl<D: DelimitersType> IntoIterator for Group<D> {
    type IntoIter = <Vec<TokenTree> as IntoIterator>::IntoIter;
    type Item = TokenTree;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}
