use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "a module"]
pub struct Mod {
    pub vis: Vis,
    pub ident: Ident,
    pub content: Option<ModContent>,
}

#[derive(Debug, Clone, Desc)]
#[desc = "module content"]
pub struct ModContent {
    pub items: Vec<Item>,
}

impl Parse for ModContent {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        Self {
            items: parser
                .try_parse_rep_all(context)
                .into_iter()
                .filter_map(|item| {
                    if let Ok(item) = item {
                        Some(item)
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

impl ItemType for Mod {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
    ) -> Result<Self, ()> {
        let vis = modifiers.take_vis();

        let _ = parser.try_parse::<keyword!("mod")>(context)?;
        let ident = parser.try_parse(context)?;
        let content = if let Some(group) = parser.parse::<Option<Group<Braces>>>(context) {
            Some(group.into_parser().parse(context))
        } else {
            let _ = parser.try_parse::<punct!(";")>(context)?;
            None
        };

        Ok(Self {
            vis,
            ident,
            content,
        })
    }
}

impl Peek for Mod {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<keyword!("mod")>(context)
    }
}
