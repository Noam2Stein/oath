use crate::*;

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a struct"]
pub struct Struct {
    pub vis: Vis,
    pub ident: Try<Ident>,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub fields: Try<Fields>,
}

#[derive(Debug, Clone, ParseDesc)]
#[desc = "fields"]
pub enum Fields {
    Named(Vec<NamedField>),
    Unnamed(Vec<UnnamedField>),
}

#[derive(Debug, Clone, ParseDesc)]
#[desc = "a named fiend"]
pub struct NamedField {
    pub vis: Vis,
    pub ident: Try<Ident>,
    pub type_: Try<Expr>,
    pub bounds: Option<Try<Expr>>,
}

#[derive(Debug, Clone, ParseDesc)]
#[desc = "an unnamed fiend"]
pub struct UnnamedField {
    pub vis: Vis,
    pub type_: Try<Expr>,
    pub bounds: Option<Try<Expr>>,
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
                    fields: Try::Failure,
                }
            }
        };

        let generics = Parse::parse(parser);
        let contract = Contract::parse(parser);

        if contract.is_not_empty() {
            let fields = Group::<Braces>::try_parse(parser).map(|group| {
                Fields::Named(
                    group
                        .into_parser(parser.context())
                        .parse_trl::<_, punct!(",")>(),
                )
            });

            Self {
                vis,
                ident,
                generics,
                contract,
                fields,
            }
        } else {
            let fields = Fields::try_parse(parser);

            let contract = if let Try::Success(Fields::Unnamed(_)) = fields {
                Parse::parse(parser)
            } else {
                contract
            };

            if let Try::Success(Fields::Unnamed(_)) = fields {
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

impl TryParse for Fields {
    fn try_parse(parser: &mut Parser<impl ParserIterator>) -> Try<Self> {
        if let Some(group) = <Option<Group<Braces>>>::parse(parser) {
            Try::Success(Self::Named(
                group
                    .into_parser(parser.context())
                    .parse_trl::<_, punct!(",")>()
                    .into_iter()
                    .collect(),
            ))
        } else if let Some(group) = <Option<Group<Parens>>>::parse(parser) {
            Try::Success(Self::Unnamed(
                group
                    .into_parser(parser.context())
                    .parse_trl::<_, punct!(",")>()
                    .into_iter()
                    .collect(),
            ))
        } else {
            parser.context().push_error(SyntaxError::Expected(
                parser.peek_span(),
                "either `{ }` or `( )`",
            ));

            Try::Failure
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
                    type_: Try::Failure,
                    bounds: None,
                };
            }
        };

        let type_ = if let Some(_) = <Option<punct!("-")>>::parse(parser) {
            Expr::try_parse_no_mhs(parser)
        } else {
            parser.context().push_error(SyntaxError::Expected(
                parser.peek_span(),
                "`ParamIdent-ParamType`",
            ));

            Try::Failure
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
        Ident::detect(parser) || <keyword!("pub")>::detect(parser)
    }
}

impl Parse for UnnamedField {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let vis = Vis::parse(parser);

        let type_ = Expr::try_parse_no_mhs(parser);

        let bounds = <Option<punct!(":")>>::parse(parser).map(|_| Parse::parse(parser));

        Self { vis, type_, bounds }
    }
}

impl Detect for UnnamedField {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        Expr::detect(parser) || <keyword!("pub")>::detect(parser)
    }
}
