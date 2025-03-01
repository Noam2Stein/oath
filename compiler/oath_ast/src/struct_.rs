use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "a struct"]
pub struct Struct {
    pub vis: Vis,
    pub ident: Ident,
    pub generics: GenericParams,
    pub contract: Contract,
    pub fields: (),
}

impl ItemParse for Struct {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
        mut target_kind: ItemKind,
    ) -> PResult<Self> {
        let vis = modifiers.take_vis();

        target_kind.expect_empty(context, Self::desc());

        let ident = parser.try_parse(context)?;
        let generics = parser.parse(context);
        let contract = parser.parse(context);

        let _ = parser.try_parse::<Group<Braces>>(context);

        Ok(Self {
            contract,
            fields: (),
            generics,
            ident,
            vis,
        })
    }
}

impl Peek for Struct {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<keyword!("struct")>(context)
    }
}
