use crate::*;

pub struct Mod {
    pub vis: Vis,
    pub mod_keyword: keyword!("mod"),
    pub ident: Ident,
    pub content: Option<ModContent>,
}

pub struct ModContent {
    pub items: Vec<Item>,
}

impl ItemType for Mod {
    const DESC: &str = "a module";

    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
    ) -> Result<Self, ()> {
        let vis = modifiers.take_vis();

        let mod_keyword = parser.parse(context)?;

        let ident = parser.parse(context)?;

        let content = if parser.peek::<Group<Braces>>(context) {
            Some(parser.parse::<InBraces<_>>(context).unwrap().inner)
        } else {
            parser.parse::<punct!(";")>(context);
            None
        };

        Ok(Self {
            vis,
            mod_keyword,
            ident,
            content,
        })
    }

    fn item_peek(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> bool {
        parser.peek::<keyword!("mod")>(context)
    }
}

impl Parse for ModContent {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        Ok(Self {
            items: parser.parse_vec_all::<_, false>(context)?,
        })
    }
}
