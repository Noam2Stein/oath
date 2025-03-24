use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a module"]
pub struct Mod {
    pub vis: Vis,
    pub ident: Try<Ident>,
    pub content: Option<ModContent>,
}

#[derive(Debug, Clone, ParseDesc)]
#[desc = "module content"]
pub struct ModContent {
    pub items: Vec<Item>,
}

impl Parse for ModContent {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        Self {
            items: parser.parse_rep(),
        }
    }
}

impl ItemParse for Mod {
    fn item_parse(
        parser: &mut Parser<impl ParserIterator>,
        modifiers: &mut ItemModifiers,
        target_kind: ItemKind,
    ) -> Self {
        let vis = modifiers.take_vis();

        target_kind.expect_empty(parser.context(), Self::desc());

        let ident = match Parse::parse(parser) {
            Try::Success(success) => Try::Success(success),
            Try::Failure => {
                return Self {
                    vis,
                    ident: Try::Failure,
                    content: None,
                }
            }
        };

        let content = if let Some(group) = <Option<Group<Braces>>>::parse(parser) {
            Some(Parse::parse(&mut group.into_parser(parser.context())))
        } else {
            <Try<punct!(";")>>::parse(parser);
            None
        };

        Self {
            vis,
            ident,
            content,
        }
    }
}
