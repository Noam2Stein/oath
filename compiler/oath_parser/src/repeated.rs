use crate::{Parse, ParseExt, Peek};

pub struct Repeated<T: Peek>(pub Vec<T>);

impl<T: Peek> Parse for Repeated<T> {
    fn parse(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = oath_tokenizer::TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut vec = vec![tokens.parse(diagnostics)];

        while let Some(value) = tokens.parse_if(diagnostics) {
            vec.push(value);
        }

        Self(vec)
    }
}
impl<T: Peek> Peek for Repeated<T> {
    fn peek(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = oath_tokenizer::TokenTree>>,
    ) -> bool {
        T::peek(tokens)
    }
}
