use crate::*;

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
        context: ContextHandle,
    ) -> Result<Self, ()> {
        let group = parser.parse::<Group<D>>(context)?;

        let delimiters = group.delimiters;
        let inner = group.into_parser().parse_all(context)?;

        Ok(Self { delimiters, inner })
    }
}

impl<T: Parse, D: DelimitersType> Peek for InDelimiters<T, D> {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        Group::<D>::peek(parser, context)
    }
}
