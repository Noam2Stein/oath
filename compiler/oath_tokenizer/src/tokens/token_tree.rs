use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Keyword(Keyword),
    Literal(Literal),
    Punct(Punct),
}

impl TokenType for TokenTree {}
impl Seal for TokenTree {}

impl<'a> TryFrom<&'a TokenTree> for TokenTree {
    type Error = ();
    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        Ok(value.clone())
    }
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
