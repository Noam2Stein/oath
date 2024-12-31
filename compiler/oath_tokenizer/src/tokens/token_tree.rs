use oath_src::{Span, Spanned};

use super::{Group, Ident, Keyword, Literal, Punct};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Keyword(Keyword),
    Literal(Literal),
    Punct(Punct),
}
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
