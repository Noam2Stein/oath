use std::iter::Peekable;

use oath_diagnostics::{Desc, DiagnosticsHandle, Error, Fill};
use oath_src::{Span, Spanned};
use oath_tokenizer::{
    CharLiteral, FloatLiteral, Ident, IntLiteral, Keyword, Literal, Punct, StrLiteral, TokenTree,
    TokenType,
};

use crate::Parse;

macro_rules! token_impl {
    ($type:ty) => {
        impl Parse for $type {
            fn parse(
                tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
                diagnostics: DiagnosticsHandle,
            ) -> Self {
                if let Some(token) = tokens.next() {
                    let span = token.span();
                    if let Ok(output) = token.try_into() {
                        output
                    } else {
                        diagnostics.push_error(Error::Expected(<Self as Desc>::DESC), span);

                        Self::fill(span)
                    }
                } else {
                    diagnostics.push_error(Error::Expected(Span::end_of_file(), Self::DESC));

                    Self::fill(Span::end_of_file())
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
