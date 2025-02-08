use oath_diagnostics::DiagnosticsHandle;
use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

impl<T: Parse> Parse for Box<T> {
    fn parse(
        parser: &mut crate::Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        Box::new(parser.parse(diagnostics))
    }
}

impl<T: Peek> Peek for Box<T> {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        T::peek(parser)
    }
}
