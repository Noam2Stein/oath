use crate::*;

#[derive(Debug, Clone, Hash, new, Spanned)]
pub struct Group<D: DelimitersType = Delimiters> {
    #[span]
    pub delimiters: D,
    pub tokens: Vec<TokenTree>,
}

verify_token_type!(Group);
with_tokens!(
    $(verify_token_type!(Group<$delim_type>);)*
);

with_tokens!(
    $(
        impl From<Group<$delim_type>> for TokenTree {
            fn from(value: Group<$delim_type>) -> Self {
                TokenTree::Group(value.into())
            }
        }
        impl TryFrom<TokenTree> for Group<$delim_type> {
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

        impl From<Group<$delim_type>> for Group {
            fn from(value: Group<$delim_type>) -> Self {
                Self {
                    delimiters: value.delimiters.into(),
                    tokens: value.tokens,
                }
            }
        }
        impl TryFrom<Group> for Group<$delim_type> {
            type Error = ();

            fn try_from(value: Group) -> Result<Self, Self::Error> {
                Ok(Self {
                    delimiters: value.delimiters.try_into()?,
                    tokens: value.tokens,
                })
            }
        }
        impl<'a> TryFrom<&'a Group> for Group<$delim_type> {
            type Error = ();

            fn try_from(value: &'a Group) -> Result<Self, Self::Error> {
                Ok(Self {
                    delimiters: value.delimiters.try_into()?,
                    tokens: value.tokens.clone(),
                })
            }
        }
    )*
);
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

impl<D: DelimitersType> IntoIterator for Group<D> {
    type IntoIter = <Vec<TokenTree> as IntoIterator>::IntoIter;
    type Item = TokenTree;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}
