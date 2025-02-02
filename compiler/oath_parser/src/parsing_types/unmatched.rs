use std::marker::PhantomData;

use oath_diagnostics::{Desc, DiagnosticsHandle, Error};
use oath_src::{Span, Spanned};
use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

pub struct Unmatched<T: Peek + Desc>(PhantomData<T>);

impl<T: Peek + Desc> Parse for Unmatched<T> {
    fn parse(
        tokens: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        let mut span = tokens
            .next()
            .map_or(Span::end_of_file(), |token| token.span());

        while tokens.is_left() && !T::peek(tokens) {
            span = span.connect(tokens.next().unwrap().span());
        }

        diagnostics.push_error(Error::Expected(T::desc()), span);

        Self(Default::default())
    }
}
