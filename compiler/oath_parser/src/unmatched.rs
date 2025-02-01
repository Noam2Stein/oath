use std::marker::PhantomData;

use oath_diagnostics::{Desc, Error};
use oath_src::{Span, Spanned};

use crate::{Parse, Peek};

pub struct Unmatched<T: Peek + Desc>(PhantomData<T>);

impl<T: Peek + Desc> Parse for Unmatched<T> {
    fn parse(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = oath_tokenizer::TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut span = tokens
            .next()
            .map_or(Span::end_of_file(), |token| token.span());

        while tokens.peek().is_some() && !T::peek(tokens) {
            span = span.connect(tokens.next().unwrap().span());
        }

        diagnostics.push_error(Error::Expected(T::desc()), span);

        Self(Default::default())
    }
}
