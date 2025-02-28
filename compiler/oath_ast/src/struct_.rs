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

impl ItemType for Struct {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
    ) -> Result<Self, ()> {
        let vis = modifiers.take_vis();

        parser.try_parse::<keyword!("struct")>(context)?;

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
