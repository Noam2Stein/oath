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
        let mut items = vec![tokens.parse(diagnostics)];
        let mut seperators = vec![tokens.parse(diagnostics)];

        while tokens.is_left() {
            items.push(tokens.parse(diagnostics));
            seperators.push(tokens.parse(diagnostics));
        }

        Self { items, seperators }
    }
}
impl<T: Parse, S: Parse> Peek for FolEndless<T, S> {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        parser.is_left()
    }
}
