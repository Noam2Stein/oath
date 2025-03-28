use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "an impl"]
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
        target_kind: Option<ItemKind>,
        _kind_keyword: ItemKeyword,
    ) -> Self {
        if let Some(target_kind) = target_kind {
            parser.context().push_error(SyntaxError::CannotHaveTarget(
                target_kind.span(),
                Self::desc(),
            ));
        };

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
