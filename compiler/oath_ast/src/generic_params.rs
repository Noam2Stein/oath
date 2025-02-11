use crate::*;

pub struct GenericParams(pub Span, pub Vec<GenericParam>);

pub struct GenericParam {
    pub ident: Ident,
    pub kind: GenericParamKind,
}

pub enum GenericParamKind {
    Value,
    Type,
}

impl Parse for GenericParams {
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

impl Parse for GenericParam {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        let ident = parser.parse(context)?;
        let kind = parser.parse(context)?;

        Ok(Self { ident, kind })
    }
}

impl Parse for GenericParamKind {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if parser.parse::<Option<keyword!("val")>>(context)?.is_some() {
            Ok(Self::Value)
        } else {
            Ok(Self::Type)
        }
    }
}
