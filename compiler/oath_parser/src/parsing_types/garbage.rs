use std::marker::PhantomData;

use oath_diagnostics::{Desc, DiagnosticsHandle, Error};
use oath_tokenizer::TokenTree;

use crate::{parse_garbage, Parse, Parser, Peek};

pub struct Garbage<T: Peek + Desc>(PhantomData<T>);

impl<T: Peek + Desc> Parse for Garbage<T> {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        diagnostics.push_error(Error::Expected(T::desc()), parse_garbage(parser, T::peek));

        Self(Default::default())
    }
}

impl<T: Peek + Desc> Default for Garbage<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
