use crate::{Parse, ParseExt, Peek};

pub struct Followed<T: Peek, S: Parse> {
    pub items: Vec<T>,
    pub seperators: Vec<S>,
}
impl<T: Peek, S: Parse> Parse for Followed<T, S> {
    fn parse(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = oath_tokenizer::TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut items = Vec::new();
        let mut seperators = Vec::new();

        while let Some(item) = tokens.parse_if(diagnostics) {
            items.push(item);
            seperators.push(tokens.parse(diagnostics));
        }

        Self { items, seperators }
    }
}
impl<T: Peek, S: Parse> Peek for Followed<T, S> {
    fn peek(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = oath_tokenizer::TokenTree>>,
    ) -> bool {
        T::peek(tokens)
    }
}
