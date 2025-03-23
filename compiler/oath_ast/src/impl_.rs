use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "an impl"]
pub struct Impl {
    pub generics: Option<GenericParams>,
    pub item: Expr,
    pub target: Option<Expr>,
    pub contract: Contract,
    pub items: BracesOrSemi<ModContent>,
}

impl ItemParse for Impl {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        _modifiers: &mut ItemModifiers,
        target_kind: ItemKind,
    ) -> PResult<Self> {
        target_kind.expect_empty(context, Self::desc());

        let generics = parser.parse(context);
        let item = parser.parse(context);
        let target = if let Some(_) = parser.parse::<Option<keyword!("for")>>(context) {
            Some(parser.parse(context))
        } else {
            None
        };
        let contract = parser.parse(context);

        let items = parser.parse(context);

        Ok(Self {
            item,
            target,
            generics,
            contract,
            items,
        })
    }
}
