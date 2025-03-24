use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "generic params"]
pub struct GenericParams(pub Vec<GenericParam>, pub Span);

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a generic param"]
pub struct GenericParam {
    pub ident: Try<Ident>,
    pub type_: Expr,
    pub bounds: Option<Expr>,
}

impl Parse for GenericParams {
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

impl Detect for GenericParams {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Group::<Angles>::detect(parser)
    }
}

impl Spanned for GenericParams {
    fn span(&self) -> Span {
        self.1
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
                    type_: Expr::Unknown(parser.peek_span()),
                    bounds: None,
                };
            }
        };

        let type_ = if let Some(_) = <Option<punct!("-")>>::parse(parser) {
            Expr::parse(parser)
        } else {
            parser.context().push_error(SyntaxError::Expected(
                parser.peek_span(),
                "`Param_Ident-Param_Type`",
            ));

            Expr::Unknown(parser.peek_span())
        };

        let bounds = parser
            .parse::<Option<punct!(":")>>()
            .map(|_| parser.parse());

        Self {
            ident,
            type_,
            bounds,
        }
    }
}

impl Detect for GenericParam {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Ident::detect(parser)
    }
}
