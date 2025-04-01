use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, new)]
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

impl Spanned for TokenFile {
    fn span(&self) -> Span {
        let start = Position::ZERO;

        let end = if let Some(last_token) = self.tokens.last() {
            last_token.span().end()
        } else {
            Position::ZERO
        };

        Span::from_start_end(start, end)
    }
}
