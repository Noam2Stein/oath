use oath_src::Span;
use oath_tokenizer::{DelimitersType, Group, TokenFile, TokenTree};

use crate::Parser;

pub trait IntoParser: IntoIterator<Item = TokenTree> {
    fn into_parser(self) -> Parser<Self::IntoIter>;
}

impl<D: DelimitersType> IntoParser for Group<D> {
    fn into_parser(self) -> Parser<Self::IntoIter> {
        Parser::new(
            self.tokens.into_iter().peekable(),
            self.delimiters.close_span(),
        )
    }
}

impl IntoParser for TokenFile {
    fn into_parser(self) -> Parser<Self::IntoIter> {
        Parser::new(self.into_iter().peekable(), Span::end_of_file())
    }
}
