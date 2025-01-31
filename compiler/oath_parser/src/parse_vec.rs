use crate::{Parse, ParseExt, Peek};

impl<T: Peek> Parse for Vec<T> {
    fn parse(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = oath_tokenizer::TokenTree>>,
        diagnostics: oath_diagnostics::DiagnosticsHandle,
    ) -> Self {
        let mut output = Vec::new();
        while let Some(value) = tokens.parse_if(diagnostics) {
            output.push(value);
        }

        output
    }
}
