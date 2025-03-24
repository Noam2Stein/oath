use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a spec"]
pub struct Sys {
    pub vis: Vis,
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
}

impl ItemParse for Sys {
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
                    generics: None,
                    contract: Contract::default(),
                }
            }
        };

        let generics = Parse::parse(parser);
        let contract = Parse::parse(parser);

        <Try<punct!(";")>>::parse(parser);

        Self {
            vis,
            ident,
            generics,
            contract,
        }
    }
}
