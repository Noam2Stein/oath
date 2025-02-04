use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rep<T: Peek>(pub Vec<T>);

impl<T: Peek> Parse for Rep<T> {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = oath_tokenizer::TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut vec = vec![parser.parse(diagnostics)];

        while let Some(value) = parser.parse(diagnostics) {
            vec.push(value);
        }

        Self(vec)
    }
}
impl<T: Peek> Peek for Rep<T> {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        T::peek(parser)
    }
}
