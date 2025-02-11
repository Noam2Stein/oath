use crate::*;

macro_rules! token_impl {
    ($type:ty => $desc:expr) => {
        impl Parse for $type {
            fn parse(
                parser: &mut Parser<impl Iterator<Item = TokenTree>>,
                context: ContextHandle,
            ) -> Result<Self, ()> {
                if let Some(token) = parser.next() {
                    let span = token.span();
                    if let Ok(output) = token.try_into() {
                        Ok(output)
                    } else {
                        context.push_error(Error::new($desc, span));

                        Err(())
                    }
                } else {
                    context.push_error(Error::new($desc, parser.end_span()));

                    Err(())
                }
            }
        }

        impl Peek for $type {
            fn peek(
                tokens: &mut Parser<impl Iterator<Item = TokenTree>>,
                _context: ContextHandle,
            ) -> bool {
                if let Some(token) = tokens.peek_next() {
                    Self::try_from(token.clone()).is_ok()
                } else {
                    false
                }
            }
        }
    };
}
token_impl!(TokenTree => "expected a token tree");
token_impl!(Keyword => "expected a keyword");
token_impl!(Punct => "expected a punct");
token_impl!(Literal => "expected a literal");
token_impl!(IntLiteral => "expected an int literal");
token_impl!(FloatLiteral => "expected a float literal");
token_impl!(CharLiteral => "expected a char literal");
token_impl!(StrLiteral => "expected a string literal");
with_token_set!(
    $(
        token_impl!(oath_tokenizer::$keyword_type => concat!("expected `", $keyword, "`"));
    )*
    $(
        token_impl!(oath_tokenizer::$punct_type => concat!("expected `", $punct, "`"));
    )*
);

impl Parse for Ident {
    fn parse(
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
            context.push_error(Error::new("expected an ident", parser.end_span()));
            Err(())
        }
    }
}
impl Peek for Ident {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, _context: ContextHandle) -> bool {
        if let Some(TokenTree::Ident(_)) = parser.peek_next() {
            true
        } else {
            false
        }
    }
}

impl<D: DelimitersType> Parse for Group<D> {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if let Some(token) = parser.next() {
            let span = token.span();
            if let TokenTree::Group(Group { delimiters, tokens }) = token {
                if let Ok(delimiters) = delimiters.try_into() {
                    Ok(Group { delimiters, tokens })
                } else {
                    context.push_error(Error::new(D::EXPECTED_GROUP, span));

                    Err(())
                }
            } else {
                context.push_error(Error::new(D::EXPECTED_GROUP, span));

                Err(())
            }
        } else {
            context.push_error(Error::new(D::EXPECTED_GROUP, parser.end_span()));

            Err(())
        }
    }
}

impl<D: DelimitersType> Peek for Group<D> {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, _context: ContextHandle) -> bool {
        if let Some(TokenTree::Group(group)) = parser.peek_next() {
            D::try_from(group.delimiters).is_ok()
        } else {
            false
        }
    }
}
