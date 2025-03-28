use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a trait"]
pub struct Trait {
    pub vis: Vis,
    pub target_kind: Try<ItemKind>,
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub items: BracesOrSemi<ModContent>,
}

impl ItemParse for Trait {
    fn item_parse(
        parser: &mut Parser<impl ParserIterator>,
        modifiers: &mut ItemModifiers,
        target_kind: Option<ItemKind>,
        kind_keyword: ItemKeyword,
    ) -> Self {
        let vis = modifiers.take_vis();

        let target_kind = if let Some(target_kind) = target_kind {
            Try::Success(target_kind)
        } else {
            parser.context().push_error(SyntaxError::Expected(
                kind_keyword.span(),
                "a trait target-kind",
            ));

            Try::Failure
        };

        let ident = match Parse::parse(parser) {
            Try::Success(success) => Try::Success(success),
            Try::Failure => {
                return Self {
                    vis,
                    target_kind,
                    ident: Try::Failure,
                    generics: None,
                    contract: Contract::default(),
                    items: BracesOrSemi::Semi,
                }
            }
        };

        parser.context().highlight(ident, HighlightColor::Green);

        let generics = Parse::parse(parser);
        let contract = Parse::parse(parser);

        let items = Parse::parse(parser);

        Self {
            vis,
            target_kind,
            ident,
            generics,
            contract,
            items,
        }
    }
}
