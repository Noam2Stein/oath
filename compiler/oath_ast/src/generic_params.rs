use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "generic params"]
pub struct GenericParams(pub Span, pub Vec<Result<GenericParam, ()>>);

#[derive(Debug, Clone, Desc)]
#[desc = "a generic param"]
pub struct GenericParam {
    pub ident: Ident,
    pub kind: PResult<ItemKind>,
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
        let kind = match parser.try_parse::<Option<ItemKind>>(context) {
            Ok(Some(kind)) => {
                if kind.keywords.len() == 1 {
                    if let ItemKeyword::Type(keyword) = kind.keywords.first().unwrap() {
                        context.push_error(Error::new("explicit `type` item-type", keyword.span()));
                    }
                }
                Ok(kind)
            }
            Ok(None) => Ok(ItemKind {
                keywords: vec![ItemKeyword::Type(keyword!("type"(ident.span())))],
            }),
            Err(()) => Err(()),
        };

        Ok(Self { ident, kind })
    }
}

impl Peek for GenericParam {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Ident>(context)
    }
}

impl PeekOk for GenericParam {}
