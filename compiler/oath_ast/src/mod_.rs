use crate::*;

#[derive(Debug, Clone)]
pub struct Mod {
    pub vis: Vis,
    pub ident: Try<Ident>,
    pub content: Option<ModContent>,
}

#[derive(Debug, Clone, Default)]
pub struct ModContent {
    pub items: Vec<Item>,
}

impl Parse for ModContent {
    fn parse(parser: &mut Parser<impl ParserIterator>, output: &mut Self) -> ParseExit {
        Self {
            items: parser.parse_rep(),
        }
    }

    fn parse_error() -> Self {
        Self { items: Vec::new() }
    }
}

impl ItemType for Mod {
    fn item_parse(
        parser: &mut Parser<impl ParserIterator>,
        modifiers: &mut ItemModifiers,
        item_kind: ItemKind,
    ) -> Self {
        let _ = item_kind.expect_no_target(parser.context());

        let vis = modifiers.take_vis();

        let ident = Ident::try_parse(parser);
        if ident.is_failure() {
            return Self {
                vis,
                ident: Try::Failure,
                content: None,
            };
        }

        parser.context().highlight(ident, HighlightColor::Green);
        ident.expect_case(IdentCase::LowerCamelCase, parser.context());

        let content = if let Some(group) = <Option<Group<Braces>>>::parse(parser) {
            Some(Parse::parse(&mut group.into_parser(parser.context())))
        } else {
            <punct!(";")>::try_parse(parser);
            None
        };

        Self {
            vis,
            ident,
            content,
        }
    }
}
