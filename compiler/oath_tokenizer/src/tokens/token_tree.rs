use oath_diagnostics::{Desc, Fill};
use oath_src::{Span, Spanned};
use oath_tokenizer_proc_macros::TokenDowncast;

use crate::Seal;

use super::{Group, Ident, Keyword, Literal, Punct, TokenType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TokenDowncast)]
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Keyword(Keyword),
    Literal(Literal),
    Punct(Punct),
}

impl TokenType for TokenTree {}
impl Seal for TokenTree {}
impl Spanned for TokenTree {
    #[inline(always)]
    fn span(&self) -> Span {
        match self {
            Self::Group(token) => token.span(),
            Self::Ident(token) => token.span(),
            Self::Keyword(token) => token.span(),
            Self::Literal(token) => token.span(),
            Self::Punct(token) => token.span(),
        }
    }
}
impl Fill for TokenTree {
    fn fill(span: Span) -> Self {
        Self::Keyword(Keyword::fill(span))
    }
}
impl Desc for TokenTree {
    fn desc() -> &'static str {
        "a token tree"
    }
}
