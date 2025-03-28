use crate::*;

#[derive(Debug, Clone, Spanned, ParseDesc)]
#[desc = "generic args"]
pub struct GenericArgs(#[span] pub Span, pub Vec<Expr>);

impl Parse for GenericArgs {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let group = match <Try<Group<Angles>>>::parse(parser) {
            Try::Success(success) => success,
            Try::Failure => return Self(parser.peek_span(), Vec::new()),
        };

        Self(
            group.span(),
            group
                .into_parser(parser.context())
                .parse_trl::<_, punct!(",")>(),
        )
    }
}

impl Detect for GenericArgs {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Group::<Angles>::detect(parser)
    }
}
