use oath_diagnostics::DiagnosticsHandle;
use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Trl<T: Peek, S: Peek> {
    pub items: Vec<T>,
    pub seperators: Vec<S>,
}
impl<T: Peek, S: Peek> Parse for Trl<T, S> {
    fn parse(
        tokens: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        let mut items = vec![tokens.parse(diagnostics)];
        let mut seperators = Vec::new();

        while let Some(seperator) = tokens.parse(diagnostics) {
            seperators.push(seperator);

            if let Some(item) = tokens.parse(diagnostics) {
                items.push(item);
            }
        }

        Self { items, seperators }
    }
}
impl<T: Peek, S: Peek> Peek for Trl<T, S> {
    fn peek(tokens: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        T::peek(tokens)
    }
}
