use crate::*;

#[derive(Debug, Clone, Desc, PeekOk)]
#[desc = "generic params"]
pub struct GenericParams(pub Vec<GenericParam>, pub Span);

#[derive(Debug, Clone, Desc)]
#[desc = "a generic param"]
pub struct GenericParam {
    pub ident: PResult<Ident>,
    pub type_: Expr,
    pub bounds: Option<Expr>,
}

impl TryParse for GenericParams {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> PResult<Self> {
        let group = parser.try_parse::<Group<Angles>>(context)?;

        let span = group.span();

        Ok(Self(
            group.into_parser().parse_trl_all::<_, punct!(",")>(context),
            span,
        ))
    }
}

impl Detect for GenericParams {
    fn detect(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Group<Angles>>(context)
    }
}

impl Spanned for GenericParams {
    fn span(&self) -> Span {
        self.1
    }
}

impl Parse for GenericParam {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        let ident = match parser.try_parse(context) {
            Ok(ok) => Ok(ok),
            Err(()) => {
                parser.skip_until(|parser| parser.peek::<punct!(",")>(context));
                return Self {
                    ident: Err(()),
                    type_: Expr::Unknown(parser.peek_span()),
                    bounds: None,
                };
            }
        };

        let type_ = if let Some(_) = parser.parse::<Option<punct!("-")>>(context) {
            parser.parse(context)
        } else {
            context.push_error(SyntaxError::Expected(
                parser.peek_span(),
                "`Param_Ident-Param_Type`",
            ));

            Expr::Unknown(parser.peek_span())
        };

        let bounds = parser
            .parse::<Option<punct!(":")>>(context)
            .map(|_| parser.parse(context));

        Self {
            ident,
            type_,
            bounds,
        }
    }
}

impl Detect for GenericParam {
    fn detect(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Ident>(context)
    }
}

impl OptionParse for GenericParam {}
