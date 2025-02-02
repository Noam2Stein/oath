use oath_diagnostics::{Desc, DiagnosticsHandle, Error, Fill};
use oath_src::Spanned;
use oath_tokenizer::{
    with_keywords, with_puncts, CharLiteral, DelimitersType, FloatLiteral, Group, Ident,
    IntLiteral, Keyword, Literal, Punct, StrLiteral, TokenDowncast, TokenTree,
};

use crate::{Parse, Parser, Peek, PeekRef};

macro_rules! token_impl {
    ($type:ty) => {
        impl Parse for $type {
            fn parse(
                parser: &mut Parser<impl Iterator<Item = TokenTree>>,
                diagnostics: DiagnosticsHandle,
            ) -> Self {
                if let Some(token) = parser.next() {
                    let span = token.span();
                    if let Some(output) = token.downcast() {
                        output
                    } else {
                        diagnostics.push_error(Error::Expected(<Self as Desc>::desc()), span);

                        Self::fill(span)
                    }
                } else {
                    diagnostics.push_error(Error::Expected(Self::desc()), parser.end_span());

                    Self::fill(parser.end_span())
                }
            }
        }

        impl Peek for $type {
            fn peek(tokens: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
                if let Some(token) = tokens.peek_next() {
                    token.downcast_ref::<Self>().is_some()
                } else {
                    false
                }
            }
        }

        impl PeekRef for $type {
            fn peek_ref(tokens: &mut Parser<impl Iterator<Item = TokenTree>>) -> Option<&Self> {
                if let Some(token) = tokens.peek_next() {
                    if let Some(token) = token.downcast_ref() {
                        Some(token)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    };
}
token_impl!(TokenTree);
token_impl!(Keyword);
token_impl!(Punct);
token_impl!(Ident);
token_impl!(Literal);
token_impl!(IntLiteral);
token_impl!(FloatLiteral);
token_impl!(CharLiteral);
token_impl!(StrLiteral);
with_keywords!($(
    token_impl!(oath_tokenizer::$keyword_type);
)*);
with_puncts!($(
    token_impl!(oath_tokenizer::$punct_type);
)*);

impl<D: DelimitersType> Parse for Group<D> {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        if let Some(token) = parser.next() {
            let span = token.span();
            if let Some(Group { delimiters, tokens }) = token.downcast::<Group>() {
                if let Some(delimiters) = delimiters.downcast() {
                    Group { delimiters, tokens }
                } else {
                    diagnostics.push_error(Error::Expected(<Self as Desc>::desc()), span);

                    Self::fill(span)
                }
            } else {
                diagnostics.push_error(Error::Expected(<Self as Desc>::desc()), span);

                Self::fill(span)
            }
        } else {
            diagnostics.push_error(Error::Expected(Self::desc()), parser.end_span());

            Self::fill(parser.end_span())
        }
    }
}

impl<D: DelimitersType> Peek for Group<D> {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        if let Some(TokenTree::Group(group)) = parser.peek_next() {
            group.delimiters.downcast_ref::<D>().is_some()
        } else {
            false
        }
    }
}
