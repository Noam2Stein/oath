use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

pub struct RepEndless<T: Parse>(pub Vec<T>);

impl<T: Parse> Parse for RepEndless<T> {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut vec = vec![parser.parse(diagnostics)];

        while parser.is_left() {
            vec.push(parser.parse(diagnostics));
        }

        Self(vec)
    }
}
impl<T: Peek> Peek for RepEndless<T> {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        T::peek(parser)
    }
}
