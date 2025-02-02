use oath_diagnostics::DiagnosticsHandle;
use oath_tokenizer::{
    Angles, Braces, Brackets, Delimiters, DelimitersType, Group, Parens, TokenTree,
};

use crate::{IntoParser, Parse, Parser, Peek};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InDelimiters<T, D: DelimitersType = Delimiters> {
    pub delimiters: D,
    pub inner: T,
}

pub type InParens<T> = InDelimiters<T, Parens>;
pub type InBrackets<T> = InDelimiters<T, Brackets>;
pub type InBraces<T> = InDelimiters<T, Braces>;
pub type InAngles<T> = InDelimiters<T, Angles>;

impl<T: Parse, D: DelimitersType> Parse for InDelimiters<T, D> {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        let group = parser.parse::<Group<D>>(diagnostics);

        let delimiters = group.delimiters;
        let inner = group.into_parser().parse_all(diagnostics);

        Self { delimiters, inner }
    }
}

impl<T: Parse, D: DelimitersType> Peek for InDelimiters<T, D> {
    fn peek(tokens: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool {
        Group::<D>::peek(tokens)
    }
}
