use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

pub struct Fol<T: Peek, S: Parse> {
    pub items: Vec<T>,
    pub seperators: Vec<S>,
}
impl<T: Peek, S: Parse> Parse for Fol<T, S> {
    fn parse(
        tokens: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut items = Vec::new();
        let mut seperators = Vec::new();

        while let Some(item) = tokens.parse(diagnostics) {
            items.push(item);
            seperators.push(tokens.parse(diagnostics));
        }

        Self { items, seperators }
    }
}
impl<T: Peek, S: Parse> Peek for Fol<T, S> {
    fn peek(tokens: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        T::peek(tokens)
    }
}
