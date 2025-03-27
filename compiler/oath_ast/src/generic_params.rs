use crate::*;

#[derive(Debug, Clone, Spanned, ParseDesc)]
#[desc = "generic params"]
pub struct GenericParams(#[span] pub Span, pub Vec<GenericParam>);

#[derive(Debug, Clone, ParseDesc, Detect)]
#[desc = "a generic param"]
pub struct GenericParam {
    pub ident: Try<Ident>,
    pub type_: Try<Expr>,
    pub bounds: Option<Try<Expr>>,
}

impl Parse for GenericParams {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let group = match Group::<Angles>::try_parse(parser) {
            Try::Success(success) => success,
            Try::Failure => return Self(parser.peek_span(), Vec::new()),
        };

        let span = group.span();
        let params = group
            .into_parser(parser.context())
            .parse_trl::<_, punct!(",")>();

        Self(span, params)
    }
}
impl Detect for GenericParams {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Group::<Angles>::detect(parser)
    }
}

impl Parse for GenericParam {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let ident = match <Try<Ident>>::parse(parser) {
            Try::Success(success) => Try::Success(success),
            Try::Failure => {
                parser.skip_until(|parser| <punct!(",")>::detect(parser));

                return Self {
                    ident: Try::Failure,
                    type_: Try::Failure,
                    bounds: None,
                };
            }
        };

        let type_ = if let Some(_) = <punct!("-")>::option_parse(parser) {
            Expr::try_parse_no_mhs(parser)
        } else {
            parser.context().push_error(SyntaxError::Expected(
                parser.peek_span(),
                "`Param_Ident-Param_Type`",
            ));

            Try::Failure
        };

        let bounds = <Option<punct!(":")>>::parse(parser).map(|_| Parse::parse(parser));

        Self {
            ident,
            type_,
            bounds,
        }
    }
}
