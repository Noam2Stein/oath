use oath_diagnostics::DiagnosticsHandle;
use oath_tokenizer::TokenTree;

use crate::{Parse, Parser, Peek};

impl<T: Peek> Parse for Vec<T> {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        let mut output = Vec::new();

        while let Some(value) = parser.parse(diagnostics) {
            output.push(value);
        }

        output
    }
}
