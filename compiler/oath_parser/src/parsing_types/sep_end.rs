use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SepEnd<T: Peek, S: Peek, E: Peek> {
    pub items: Vec<T>,
    pub seperators: Vec<S>,
    pub end: Option<E>,
}
impl<T: Peek, S: Peek, E: Peek> Parse for SepEnd<T, S, E> {
    fn parse(
        tokens: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut items = vec![tokens.parse(diagnostics)];
        let mut seperators = Vec::new();

        while let Some(seperator) = tokens.parse(diagnostics) {
            seperators.push(seperator);

            if let Some(item) = tokens.parse(diagnostics) {
                items.push(item);
            } else {
                return Self {
                    items,
                    seperators,
                    end: Some(tokens.parse(diagnostics)),
                };
            }
        }

        Self {
            items,
            seperators,
            end: None,
        }
    }
}
impl<T: Peek, S: Peek, E: Peek> Peek for SepEnd<T, S, E> {
    fn peek(tokens: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        T::peek(tokens)
    }
}
