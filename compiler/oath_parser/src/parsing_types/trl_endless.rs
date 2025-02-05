use oath_diagnostics::DiagnosticsHandle;
use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrlEndless<T: Parse, S: Parse> {
    pub items: Vec<T>,
    pub seperators: Vec<S>,
}
impl<T: Parse, S: Parse> Parse for TrlEndless<T, S> {
    fn parse(
        tokens: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        let mut items = vec![tokens.parse(diagnostics)];
        let mut seperators = Vec::new();

        while tokens.is_left() {
            seperators.push(tokens.parse(diagnostics));

            if tokens.is_left() {
                items.push(tokens.parse(diagnostics));
            }
        }

        Self { items, seperators }
    }
}
impl<T: Parse, S: Parse> Peek for TrlEndless<T, S> {
    fn peek(tokens: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        tokens.is_left()
    }
}
