use crate::*;

pub struct Struct {
    pub vis: Vis,
    pub ident: Ident,
    pub generics: GenericParams,
    pub contract: Contract,
    pub fields: (),
}

impl ItemType for Struct {
    const DESC: &str = "a struct";

    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
    ) -> Result<Self, ()> {
        let vis = modifiers.take_vis();

        parser.parse::<keyword!("struct")>(context)?;

        let ident = parser.parse(context)?;
        let generics = parser.parse(context)?;
        let contract = parser.parse(context)?;

        let _ = parser.parse::<Group<Parens>>(context);

        Ok(Self {
            contract,
            fields: (),
            generics,
            ident,
            vis,
        })
    }

    fn item_peek(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> bool {
        parser.peek::<keyword!("struct")>(context)
    }
}
