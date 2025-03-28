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
        target_kind: Option<ItemKind>,
        _kind_keyword: ItemKeyword,
    ) -> Self {
        let vis = modifiers.take_vis();

        if let Some(target_kind) = target_kind {
            parser.context().push_error(SyntaxError::CannotHaveTarget(
                target_kind.span(),
                Self::desc(),
            ));
        };

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

        parser.context().highlight(ident, HighlightColor::Green);

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
