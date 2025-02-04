use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
impl<T: Parse> Peek for RepEndless<T> {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        parser.is_left()
    }
}
