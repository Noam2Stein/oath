use crate::*;

#[derive(Debug, Clone)]
pub struct Impl {
    pub generics: Option<GenericParams>,
    pub item: Try<Expr>,
    pub target: Option<Try<Expr>>,
    pub contract: Contract,
    pub items: BracesOrSemi<ModContent>,
}

impl ItemParse for Impl {
    fn item_parse(
        parser: &mut Parser<impl ParserIterator>,
        _modifiers: &mut ItemModifiers,
        item_kind: ItemKind,
    ) -> Self {
        item_kind.expect_no_target(parser.context());

        let generics = Parse::parse(parser);
        let item = Parse::parse(parser);
        let target = <Option<keyword!("for")>>::parse(parser).map(|_| Parse::parse(parser));
        let contract = Parse::parse(parser);

        let items = Parse::parse(parser);

        Self {
            item,
            target,
            generics,
            contract,
            items,
        }
    }
}
