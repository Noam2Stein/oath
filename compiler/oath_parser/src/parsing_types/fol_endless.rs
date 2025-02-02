use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

pub struct FolEndless<T: Parse, S: Parse> {
    pub items: Vec<T>,
    pub seperators: Vec<S>,
}
impl<T: Parse, S: Parse> Parse for FolEndless<T, S> {
    fn parse(
        tokens: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut items = Vec::new();
        let mut seperators = Vec::new();

        while tokens.is_left() {
            items.push(tokens.parse(diagnostics));
            seperators.push(tokens.parse(diagnostics));
        }

        Self { items, seperators }
    }
}
impl<T: Peek, S: Parse> Peek for FolEndless<T, S> {
    fn peek(tokens: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        T::peek(tokens)
    }
}
