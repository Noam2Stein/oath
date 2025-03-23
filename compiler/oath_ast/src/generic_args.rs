use crate::*;

#[derive(Debug, Clone, Desc, PeekOk)]
#[desc = "generic args"]
pub struct GenericArgs(pub Vec<Expr>, pub Span);

impl TryParse for GenericArgs {
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

impl Detect for GenericArgs {
    fn detect(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Group<Angles>>(context)
    }
}

impl Spanned for GenericArgs {
    fn span(&self) -> Span {
        self.1
    }
}
