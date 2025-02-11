use crate::*;

pub struct GenericArgs(pub Span, pub Vec<Expr>);

impl Parse for GenericArgs {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if let Some(group) = parser.parse::<Option<Group<Angles>>>(context)? {
            Ok(Self(
                group.span(),
                group
                    .into_parser()
                    .parse_sep_all::<_, punct!(","), false, true>(context)?,
            ))
        } else {
            Ok(Self(
                Span::from_start_len(parser.next_span().start(), 0),
                Vec::new(),
            ))
        }
    }
}

impl Spanned for GenericArgs {
    fn span(&self) -> Span {
        self.0
    }
}
