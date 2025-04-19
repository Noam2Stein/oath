use crate::*;

#[derive(Debug, Clone, Spanned)]
pub struct GenericParams(#[span] pub Span, pub Vec<GenericParam>);

#[derive(Debug, Clone, OptionParse)]
#[desc = "a generic param"]
pub struct GenericParam {
    pub ident: Ident,
    pub _minus: Try<punct!("-")>,
    pub type_: Try<Expr>,
    pub bounds: Option<Bounds>,
}

impl OptionParse for GenericParams {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        let group = Group::<Angles>::option_parse(parser)?;

        let span = group.span();

        let params = group
            .into_parser(parser.context())
            .parse_trl::<_, punct!(",")>();

        Some(Self(span, params))
    }

    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Group::<Angles>::detect(parser)
    }

    fn desc() -> &'static str {
        "generic params"
    }
}
