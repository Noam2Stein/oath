use crate::*;

pub trait IntoParser {
    type Iter: ParserIterator;

    fn into_parser<'ctx>(self, context: ContextHandle<'ctx>) -> Parser<'ctx, Self::Iter>;
}

impl<D: DelimitersType> IntoParser for Group<D> {
    type Iter = Vec<TokenTree>;

    fn into_parser<'ctx>(mut self, context: ContextHandle<'ctx>) -> Parser<'ctx, Self::Iter> {
        self.tokens.reverse();

        Parser::new(self.tokens, context, self.delimiters.span().start())
    }
}

impl IntoParser for TokenFile {
    type Iter = Vec<TokenTree>;

    fn into_parser<'ctx>(mut self, context: ContextHandle<'ctx>) -> Parser<'ctx, Self::Iter> {
        self.tokens.reverse();

        Parser::new(self.tokens, context, Position::new(0, 0))
    }
}
