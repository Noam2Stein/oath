use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "generic args"]
pub struct GenericArgs(pub Vec<Expr>, pub Span);

impl Parse for GenericArgs {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let group = match <Try<Group<Angles>>>::parse(parser) {
            Try::Success(success) => success,
            Try::Failure => Self(Vec::new(), parser.peek_span()),
        };

        let span = group.span();

        Self(
            group
                .into_parser(parser.context())
                .parse_trl_all::<_, punct!(",")>(),
            span,
        )
    }
}

impl Detect for GenericArgs {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Group::<Angles>::detect(parser)
    }
}

impl Spanned for GenericArgs {
    fn span(&self) -> Span {
        self.1
    }
}
