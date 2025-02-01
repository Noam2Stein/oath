use crate::{Parse, ParseExt, Peek};

pub struct Endless<T: Parse>(pub Vec<T>);

impl<T: Parse> Parse for Endless<T> {
    fn parse(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = oath_tokenizer::TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut vec = Vec::new();

        while tokens.peek().is_some() {
            vec.push(tokens.parse(diagnostics));
        }

        Self(vec)
    }
}
impl<T: Peek> Peek for Endless<T> {
    fn peek(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = oath_tokenizer::TokenTree>>,
    ) -> bool {
        T::peek(tokens)
    }
}
