use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "a spec"]
pub struct Sys {
    pub vis: Vis,
    pub ident: Ident,
    pub generics: Option<GenericParams>,
}

impl ItemParse for Sys {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
        target_kind: ItemKind,
    ) -> PResult<Self> {
        let vis = modifiers.take_vis();

        target_kind.expect_empty(context, Self::desc());

        let ident = parser.try_parse(context)?;
        let generics = parser.parse(context);

        let _ = parser.try_parse::<punct!(";")>(context);

        Ok(Self {
            vis,
            ident,
            generics,
        })
    }
}
