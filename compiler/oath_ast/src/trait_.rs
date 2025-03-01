use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "a trait"]
pub struct Trait {
    pub vis: Vis,
    pub target: ItemKind,
    pub ident: Ident,
    pub generics: GenericParams,
    pub contract: Contract,
    pub items: Option<Vec<Item>>,
}

impl ItemParse for Trait {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
        target_kind: ItemKind,
    ) -> PResult<Self> {
        let vis = modifiers.take_vis();

        let ident = parser.try_parse(context)?;
        let generics = parser.parse(context);
        let contract = parser.parse(context);

        let items = if let Some(group) = parser.parse::<Option<Group<Braces>>>(context) {
            Some(
                group
                    .into_parser()
                    .try_parse_rep_all(context)
                    .into_iter()
                    .filter_map(Result::ok)
                    .collect(),
            )
        } else {
            if let None = parser.parse::<Option<punct!(";")>>(context) {
                context.push_error(SyntaxError::Expected(
                    parser.next_span(),
                    "either `{ }` or `;`",
                ));
            }
            None
        };

        Ok(Self {
            vis,
            target: target_kind,
            ident,
            generics,
            contract,
            items,
        })
    }
}
