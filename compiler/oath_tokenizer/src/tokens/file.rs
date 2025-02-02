use super::TokenTree;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenFile {
    pub tokens: Vec<TokenTree>,
}

impl IntoIterator for TokenFile {
    type IntoIter = <Vec<TokenTree> as IntoIterator>::IntoIter;
    type Item = TokenTree;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}
