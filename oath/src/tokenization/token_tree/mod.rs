use std::fmt::{self, Display, Formatter};

mod group;
mod ident;
mod keyword;
mod literal;
mod punct;
pub use group::*;
pub use ident::*;
pub use keyword::*;
pub use literal::*;
pub use punct::*;

use super::*;

#[derive(Debug, Clone, Hash)]
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Keyword(Keyword),
    Literal(Literal),
    Punct(Punct),
}
impl Display for TokenTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Group(token_tree) => token_tree.fmt(f),
            Self::Ident(token_tree) => token_tree.fmt(f),
            Self::Keyword(token_tree) => token_tree.fmt(f),
            Self::Literal(token_tree) => token_tree.fmt(f),
            Self::Punct(token_tree) => token_tree.fmt(f),
        }
    }
}
impl Spanned for TokenTree {
    fn span(&self) -> Span {
        match self {
            Self::Group(token_tree) => token_tree.span(),
            Self::Ident(token_tree) => token_tree.span(),
            Self::Keyword(token_tree) => token_tree.span(),
            Self::Literal(token_tree) => token_tree.span(),
            Self::Punct(token_tree) => token_tree.span(),
        }
    }
}
