use crate::*;

#[derive(Debug, Clone, Spanned)]
pub struct GenericArgs(#[span] pub Span, pub Vec<Expr>);

impl OptionParse for GenericArgs {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        let group = Group::<Angles>::option_parse(parser)?;

        let span = group.span();

        let items = group
            .into_parser(parser.context())
            .parse_trl::<_, punct!(",")>();

        Some(Self(span, items))
    }

    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Group::<Angles>::detect(parser)
    }

    fn desc() -> &'static str {
        "generic args"
    }
}
