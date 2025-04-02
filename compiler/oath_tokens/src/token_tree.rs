use crate::*;

#[derive(Debug, Clone, Hash, From, TryInto, Spanned, InternedDisplay)]
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Keyword(Keyword),
    Literal(Literal),
    Punct(Punct),
}

verify_token_type!(TokenTree);

impl<'a> TryFrom<&'a TokenTree> for TokenTree {
    type Error = ();
    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        Ok(value.clone())
    }
}
