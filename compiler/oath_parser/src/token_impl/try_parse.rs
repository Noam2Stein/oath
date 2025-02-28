use crate::*;

macro_rules! impl_try_parse {
    ($ty:ty) => {
        impl TryParse for $ty {
            fn try_parse(
                parser: &mut Parser<impl Iterator<Item = TokenTree>>,
                context: ContextHandle,
            ) -> Result<Self, ()> {
                if let Some(token) = parser.next() {
                    let span = token.span();
                    #[allow(irrefutable_let_patterns)]
                    if let Ok(token) = token.try_into() {
                        Ok(token)
                    } else {
                        context.push_error(Error::new(format!("expected {}", <$ty>::desc()), span));

                        Err(())
                    }
                } else {
                    context.push_error(Error::new(
                        format!("expected {}", <$ty>::desc()),
                        parser.next_span(),
                    ));

                    Err(())
                }
            }
        }
    };
}
impl_try_parse!(TokenTree);

impl_try_parse!(Keyword);
with_token_set!($(
    impl_try_parse!($keyword_type);
)*);

impl_try_parse!(Punct);
with_token_set!($(
    impl_try_parse!($punct_type);
)*);

impl_try_parse!(Literal);
impl_try_parse!(IntLiteral);
impl_try_parse!(FloatLiteral);
impl_try_parse!(CharLiteral);
impl_try_parse!(StrLiteral);

impl_try_parse!(Group);
with_token_set!($(
    impl_try_parse!(Group<$delim_type>);
)*);

impl TryParse for Ident {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if let Some(token) = parser.next() {
            match token {
                TokenTree::Ident(token) => Ok(token),
                TokenTree::Keyword(token) => {
                    context.push_error(Error::new(
                        format!("expected an ident. `{token}` is a keyword"),
                        token.span(),
                    ));

                    Ok(Ident::new_adjusted(
                        token.kind.as_str(),
                        token.span(),
                        context,
                    ))
                }
                token => {
                    context.push_error(Error::new("expected an ident", token.span()));

                    Err(())
                }
            }
        } else {
            context.push_error(Error::new("expected an ident", parser.next_span()));

            Err(())
        }
    }
}
