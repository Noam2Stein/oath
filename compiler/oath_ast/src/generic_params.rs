use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "generic params"]
pub struct GenericParams(pub Span, pub Vec<Result<GenericParam, ()>>);

#[derive(Debug, Clone, Desc)]
#[desc = "a generic param"]
pub struct GenericParam {
    pub ident: Ident,
    pub kind: GenericParamKind,
}

#[derive(Debug, Clone, Desc)]
#[desc = "a generic param kind"]
pub enum GenericParamKind {
    Value,
    Type,
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
        let ident = parser.try_parse(context)?;
        let kind = parser.parse(context);

        Ok(Self { ident, kind })
    }
}

impl Peek for GenericParam {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Ident>(context)
    }
}

impl PeekOk for GenericParam {}

impl Parse for GenericParamKind {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        if parser.parse::<Option<keyword!("val")>>(context).is_some() {
            Self::Value
        } else {
            Self::Type
        }
    }
}
