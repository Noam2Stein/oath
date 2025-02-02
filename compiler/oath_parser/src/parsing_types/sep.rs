use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

pub struct Sep<T: Parse, S: Peek> {
    pub items: Vec<T>,
    pub seperators: Vec<S>,
}
impl<T: Parse, S: Peek> Parse for Sep<T, S> {
    fn parse(
        tokens: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut items = vec![tokens.parse(diagnostics)];
        let mut seperators = Vec::new();

        while let Some(seperator) = tokens.parse(diagnostics) {
            seperators.push(seperator);
            items.push(tokens.parse(diagnostics));
        }

        Self { items, seperators }
    }
}
impl<T: Peek, S: Peek> Peek for Sep<T, S> {
    fn peek(tokens: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        T::peek(tokens)
    }
}
