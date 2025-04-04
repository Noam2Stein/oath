use crate::*;

#[derive(Debug, Clone, Spanned)]
pub struct GenericParams(#[span] pub Span, pub Vec<GenericParam>);

#[derive(Debug, Clone)]
pub struct GenericParam {
    pub ident: Try<Ident>,
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

        parser.context().highlight(ident, HighlightColor::Green);
        ident.expect_case(IdentCase::UpperCamelCase, parser.context());

        let type_ = if let Some(_) = <punct!("-")>::option_parse(parser) {
            Expr::try_parse_no_mhs(parser)
        } else {
            parser.context().push_error(SyntaxError::Expected(
                parser.peek_span(),
                "`Param_Ident-Param_Type`",
            ));

            Try::Failure
        };

        let bounds = Bounds::option_parse(parser);

        Self {
            ident,
            type_,
            bounds,
        }
    }

    fn parse_error() -> Self {
        Self {
            ident: Parse::parse_error(),
            type_: Parse::parse_error(),
            bounds: Parse::parse_error(),
        }
    }
}
