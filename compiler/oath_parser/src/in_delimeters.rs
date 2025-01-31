use std::iter::Peekable;

use oath_diagnostics::DiagnosticsHandle;
use oath_tokenizer::{Braces, Brackets, Delimiters, DelimitersType, Group, Parens, TokenTree};

use crate::{Parse, ParseExt};

pub struct InDelimiters<T, D: DelimitersType = Delimiters> {
    pub delimiters: D,
    pub inner: T,
}

pub type InParens<T> = InDelimiters<T, Parens>;
pub type InBrackets<T> = InDelimiters<T, Brackets>;
pub type InBraces<T> = InDelimiters<T, Braces>;

impl<T: Parse, D: DelimitersType> Parse for InDelimiters<T, D> {
    fn parse(
        tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        let Group { delimiters, tokens } = tokens.parse::<Group<D>>(diagnostics);
        let inner = tokens.into_iter().peekable().parse(diagnostics);

        Self { delimiters, inner }
    }
}
