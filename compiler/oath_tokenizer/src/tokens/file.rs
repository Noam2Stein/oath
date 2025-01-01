use super::TokenTree;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenFile {
    pub tokens: Vec<TokenTree>,
}
