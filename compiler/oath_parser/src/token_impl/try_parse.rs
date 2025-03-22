use crate::*;

macro_rules! impl_try_parse {
    ($ty:ty) => {
        impl TryParse for $ty {
            fn try_parse(
                parser: &mut Parser<impl Iterator<Item = TokenTree>>,
                context: ContextHandle,
            ) -> Result<Self, ()> {
                if parser.peek::<Self>(context) {
                    Ok(parser.next().unwrap().try_into().unwrap())
                } else {
                    context.push_error(Error::new(
                        format!("Syntax Error: expected {}", <$ty>::desc()),
                        parser.peek_span(),
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
        if let Some(token) = parser.peek_next() {
            match token {
                TokenTree::Ident(_) => Ok(parser.next().unwrap().try_into().unwrap()),
                TokenTree::Keyword(token) => {
                    context.push_error(Error::new(
                        format!("Syntax Error: expected an ident. `{token}` is a keyword"),
                        parser.peek_span(),
                    ));

                    Err(())
                }
                _ => {
                    context.push_error(Error::new(
                        "Syntax Error: expected an ident",
                        parser.peek_span(),
                    ));

                    Err(())
                }
            }
        } else {
            context.push_error(Error::new(
                "Syntax Error: expected an ident",
                parser.peek_span(),
            ));

            Err(())
        }
    }
}
