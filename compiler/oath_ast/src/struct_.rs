use crate::*;

#[derive(Debug, Clone, Parse)]
pub struct Struct {
    #[dont_parse]
    pub vis: Vis,
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub fields: Try<Fields>,
}

#[derive(Debug, Clone)]
pub enum Fields {
    Named(Vec<NamedField>),
    Unnamed(Vec<UnnamedField>),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a named field"]
pub struct NamedField {
    pub ident: Ident,
    pub _minus: Try<punct!("-")>,
    pub type_: Try<Expr>,
    pub bounds: Option<Bounds>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "an unnamed field"]
pub struct UnnamedField {
    pub type_: Expr,
    pub bounds: Option<Bounds>,
}

impl ItemType for Struct {
    fn add_modifiers(
        &mut self,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
        item_kind: ItemKind,
    ) {
        item_kind.expect_no_target(context);

        self.vis = modifiers.take_vis();
    }
}

impl OptionParse for Fields {
    fn option_parse(
        parser: &mut Parser<impl ParserIterator>,
        output: &mut Option<Self>,
    ) -> ParseExit {
        if let Some(group) = Group::<Braces>::option_parse(parser) {
            Some(Self::Named(
                group
                    .into_parser(parser.context())
                    .parse_trl::<_, punct!(",")>()
                    .into_iter()
                    .collect(),
            ))
        } else if let Some(group) = Group::<Parens>::option_parse(parser) {
            Some(Self::Unnamed(
                group
                    .into_parser(parser.context())
                    .parse_trl::<_, punct!(",")>()
                    .into_iter()
                    .collect(),
            ))
        } else {
            None
        }
    }

    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Group::<Braces>::detect(parser) || Group::<Parens>::detect(parser)
    }

    fn desc() -> &'static str {
        "either `{ }` or `( )`"
    }
}
