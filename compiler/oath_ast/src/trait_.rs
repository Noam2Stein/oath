use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "a trait"]
pub struct Trait {
    pub vis: Vis,
    pub target: ItemKind,
    pub ident: Ident,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub items: BracesOrSemi<ModContent>,
}

impl ItemParse for Trait {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
        target_kind: ItemKind,
    ) -> PResult<Self> {
        let vis = modifiers.take_vis();

        let ident = parser.try_parse(context)?;
        let generics = parser.parse(context);
        let contract = parser.parse(context);
        let items = parser.parse(context);

        Ok(Self {
            vis,
            target: target_kind,
            ident,
            generics,
            contract,
            items,
        })
    }
}
