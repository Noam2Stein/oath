use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "generic params"]
pub struct GenericParams(pub Span, pub Vec<Result<GenericParam, ()>>);

#[derive(Debug, Clone, Desc)]
#[desc = "a generic param"]
pub struct GenericParam {
    pub ident: Ident,
    pub type_: PResult<Expr>,
    pub bounds: Option<Expr>,
}

impl Parse for GenericParams {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        if let Some(group) = parser.parse::<Option<Group<Angles>>>(context) {
            Self(
                group.span(),
                group
                    .into_parser()
                    .try_parse_trl_all::<_, punct!(",")>(context),
            )
        } else {
            Self(
                Span::from_start_len(parser.next_span().start(), 0),
                Vec::new(),
            )
        }
    }
}

impl TryParse for GenericParam {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        let ident = parser.try_parse::<Ident>(context)?;

        let type_ = if let Some(_) = parser.parse::<Option<punct!("-")>>(context) {
            parser.try_parse(context)
        } else {
            context.push_error(SyntaxError::Expected(
                parser.next_span(),
                "`Param_Ident-Param_Type`",
            ));
            Err(())
        };

        let bounds = if let Some(_) = parser.parse::<Option<punct!(":")>>(context) {
            parser.try_parse(context).ok()
        } else {
            None
        };

        Ok(Self {
            ident,
            type_,
            bounds,
        })
    }
}

impl Peek for GenericParam {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Ident>(context)
    }
}

impl PeekOk for GenericParam {}
