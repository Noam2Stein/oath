use oath_diagnostics::DiagnosticsHandle;
use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SepEndless<T: Parse, S: Parse> {
    pub items: Vec<T>,
    pub seperators: Vec<S>,
}
impl<T: Parse, S: Parse> Parse for SepEndless<T, S> {
    fn parse(
        tokens: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        let mut items = vec![tokens.parse(diagnostics)];
        let mut seperators = Vec::new();

        while tokens.is_left() {
            seperators.push(tokens.parse(diagnostics));
            items.push(tokens.parse(diagnostics));
        }

        Self { items, seperators }
    }
}
impl<T: Peek, S: Parse> Peek for SepEndless<T, S> {
    fn peek(tokens: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        T::peek(tokens)
    }
}
