use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "generic args"]
pub struct GenericArgs(pub Span, pub Vec<Result<Expr, ()>>);

impl Parse for GenericArgs {
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

impl Spanned for GenericArgs {
    fn span(&self) -> Span {
        self.0
    }
}
