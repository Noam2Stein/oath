use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a struct"]
pub struct Struct {
    pub vis: Vis,
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub fields: Fields,
}

#[derive(Debug, Clone, ParseDesc)]
#[desc = "fields"]
pub enum Fields {
    Named(Vec<NamedField>),
    Unnamed(Vec<UnnamedField>),
    Unknown,
}

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a named fiend"]
pub struct NamedField {
    pub vis: Vis,
    pub ident: Try<Ident>,
    pub type_: Expr,
    pub bounds: Option<Expr>,
}

#[derive(Debug, Clone, ParseDesc)]
#[desc = "an unnamed fiend"]
pub struct UnnamedField {
    pub vis: Vis,
    pub type_: Expr,
    pub bounds: Option<Expr>,
}

impl ItemParse for Struct {
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
                    fields: Fields::Unknown,
                }
            }
        };

        let generics = Parse::parse(parser);
        let contract = Contract::parse(parser);

        if contract.is_not_empty() {
            let fields = if let Try::Success(group) = <Try<Group<Braces>>>::parse(parser) {
                Fields::Named(
                    group
                        .into_parser(parser.context())
                        .parse_trl::<_, punct!(",")>(),
                )
            } else {
                Fields::Unknown
            };

            Self {
                vis,
                ident,
                generics,
                contract,
                fields,
            }
        } else {
            let fields = Fields::parse(parser);

            let contract = if let Fields::Unnamed(_) = fields {
                Parse::parse(parser)
            } else {
                contract
            };

            if let Fields::Unnamed(_) = fields {
                <Try<punct!(";")>>::parse(parser);
            };

            Self {
                vis,
                ident,
                generics,
                contract,
                fields,
            }
        }
    }
}

impl Parse for Fields {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        if let Some(group) = <Option<Group<Braces>>>::parse(parser) {
            Self::Named(
                group
                    .into_parser(parser.context())
                    .parse_trl::<_, punct!(",")>()
                    .into_iter()
                    .collect(),
            )
        } else if let Some(group) = <Option<Group<Parens>>>::parse(parser) {
            Self::Unnamed(
                group
                    .into_parser(parser.context())
                    .parse_trl::<_, punct!(",")>()
                    .into_iter()
                    .collect(),
            )
        } else {
            parser.context().push_error(SyntaxError::Expected(
                parser.peek_span(),
                "either `{ }` or `( )`",
            ));

            Self::Unknown
        }
    }
}

impl Parse for NamedField {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let vis = Vis::parse(parser);

        let ident = match Parse::parse(parser) {
            Try::Success(success) => Try::Success(success),
            Try::Failure => {
                parser.skip_until(|parser| <punct!(",")>::detect(parser));

                return Self {
                    vis,
                    ident: Try::Failure,
                    type_: Expr::Unknown,
                    bounds: None,
                };
            }
        };

        let type_ = if let Some(_) = <Option<punct!("-")>>::parse(parser) {
            Expr::parse_no_mhs(parser)
        } else {
            parser.context().push_error(SyntaxError::Expected(
                parser.peek_span(),
                "`ParamIdent-ParamType`",
            ));

            Expr::Unknown
        };

        let bounds = <Option<punct!(":")>>::parse(parser).map(|_| Parse::parse(parser));

        Self {
            vis,
            ident,
            type_,
            bounds,
        }
    }
}

impl Detect for NamedField {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Ident::detect(parser) || Vis::detect(parser)
    }
}

impl Parse for UnnamedField {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let vis = Vis::parse(parser);

        let type_ = Expr::parse_no_mhs(parser);

        let bounds = <Option<punct!(":")>>::parse(parser).map(|_| Parse::parse(parser));

        Self { vis, type_, bounds }
    }
}

impl Detect for UnnamedField {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Expr::detect(parser) || Vis::detect(parser)
    }
}
