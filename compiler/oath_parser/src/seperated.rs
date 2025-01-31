use crate::{Parse, ParseExt, Peek};

pub struct Seperated<T: Parse, S: Peek> {
    pub items: Vec<T>,
    pub seperators: Vec<S>,
}
impl<T: Parse, S: Peek> Parse for Seperated<T, S> {
    fn parse(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = oath_tokenizer::TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut items = vec![tokens.parse(diagnostics)];
        let mut seperators = Vec::new();

        while let Some(seperator) = tokens.parse_if(diagnostics) {
            seperators.push(seperator);
            items.push(tokens.parse(diagnostics));
        }

        Self { items, seperators }
    }
}
impl<T: Peek, S: Peek> Peek for Seperated<T, S> {
    fn peek(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = oath_tokenizer::TokenTree>>,
    ) -> bool {
        T::peek(tokens)
    }
}
