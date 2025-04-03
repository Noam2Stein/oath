use crate::*;

#[derive(Debug, Clone, Parse)]
pub struct Struct {
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

#[derive(Debug, Clone)]
pub struct NamedField {
    pub vis: Vis,
    pub ident: Try<Ident>,
    pub type_: Try<Expr>,
    pub bounds: Option<Bounds>,
}

#[derive(Debug, Clone, ParseDesc, ParseError, Detect)]
#[desc = "an unnamed fiend"]
pub struct UnnamedField {
    #[option_detect]
    pub vis: Vis,
    pub type_: Try<Expr>,
    pub bounds: Option<Bounds>,
}

impl ItemParse for Struct {
    fn item_parse(
        parser: &mut Parser<impl ParserIterator>,
        modifiers: &mut ItemModifiers,
        item_kind: ItemKind,
    ) -> Self {
        item_kind.expect_no_target(parser.context());

        let vis = modifiers.take_vis();

        let ident = Ident::try_parse(parser);
        if ident.is_failure() {
            return Self {
                vis,
                ident: Try::Failure,
                generics: None,
                contract: Contract::default(),
                fields: Try::Failure,
            };
        }

        parser.context().highlight(ident, HighlightColor::Green);
        ident.expect_case(IdentCase::UpperCamelCase, parser.context());

        let generics = Parse::parse(parser);
        let mut contract = Contract::parse(parser);

        let fields = if contract.is_not_empty() {
            Group::<Braces>::try_parse(parser).map(|group| {
                Fields::Named(
                    group
                        .into_parser(parser.context())
                        .parse_trl::<_, punct!(",")>(),
                )
            })
        } else {
            Fields::try_parse(parser)
        };

        if let Try::Success(Fields::Unnamed(_)) = fields {
            contract = Parse::parse(parser);

            <punct!(";")>::try_parse(parser);
        };

        if fields.is_failure() {
            parser.skip_until(Item::detect);
        }

        Self {
            vis,
            ident,
            generics,
            contract,
            fields,
        }
    }
}

impl OptionParse for Fields {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
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
}
impl Detect for Fields {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Group::<Braces>::detect(parser) || Group::<Parens>::detect(parser)
    }
}

impl Parse for NamedField {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let vis = Vis::parse(parser);

        let ident = Ident::try_parse(parser);
        if ident.is_failure() {
            parser.skip_until(|parser| <punct!(",")>::detect(parser));

            return Self {
                vis,
                ident: Try::Failure,
                type_: Try::Failure,
                bounds: None,
            };
        }

        parser.context().highlight(ident, HighlightColor::Cyan);
        ident.expect_case(IdentCase::LowerCamelCase, parser.context());

        let type_ = if let Some(_) = <Option<punct!("-")>>::parse(parser) {
            Expr::try_parse_no_mhs(parser)
        } else {
            parser.context().push_error(SyntaxError::Expected(
                parser.peek_span(),
                "`ParamIdent-ParamType`",
            ));

            Try::Failure
        };

        let bounds = Bounds::option_parse(parser);

        Self {
            vis,
            ident,
            type_,
            bounds,
        }
    }
}

impl Parse for UnnamedField {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let vis = Vis::parse(parser);

        let type_ = Expr::try_parse_no_mhs(parser);

        let bounds = Bounds::option_parse(parser);

        Self { vis, type_, bounds }
    }
}
