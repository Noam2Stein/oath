use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "a struct"]
pub struct Struct {
    pub vis: Vis,
    pub ident: Ident,
    pub generics: Option<GenericParams>,
    pub contract: Contract,
    pub fields: Fields,
}

#[derive(Debug, Clone, Desc)]
#[desc = "fields"]
pub enum Fields {
    Named(Vec<NamedField>),
    Unnamed(Vec<UnnamedField>),
    Unknown,
}

#[derive(Debug, Clone, Desc)]
#[desc = "a named fiend"]
pub struct NamedField {
    pub vis: Vis,
    pub ident: PResult<Ident>,
    pub type_: Expr,
    pub bounds: Option<Expr>,
}

#[derive(Debug, Clone, Desc)]
#[desc = "an unnamed fiend"]
pub struct UnnamedField {
    pub vis: Vis,
    pub type_: Expr,
    pub bounds: Option<Expr>,
}

impl ItemParse for Struct {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
        target_kind: ItemKind,
    ) -> PResult<Self> {
        let vis = modifiers.take_vis();

        target_kind.expect_empty(context, Self::desc());

        let ident = parser.try_parse(context)?;
        let generics = parser.parse(context);
        let contract = parser.parse::<Contract>(context);

        if contract.is_not_empty() {
            let fields = if let Ok(group) = parser.try_parse::<Group<Braces>>(context) {
                Fields::Named(group.into_parser().parse_trl_all::<_, punct!(",")>(context))
            } else {
                Fields::Unknown
            };

            Ok(Self {
                vis,
                ident,
                generics,
                contract,
                fields,
            })
        } else {
            let fields = parser.parse(context);

            let contract = if let Fields::Unnamed(_) = fields {
                parser.parse(context)
            } else {
                contract
            };

            if let Fields::Unnamed(_) = fields {
                let _ = parser.try_parse::<punct!(";")>(context);
            };

            Ok(Self {
                vis,
                ident,
                generics,
                contract,
                fields,
            })
        }
    }
}

impl Detect for Struct {
    fn detect(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> bool {
        parser.peek::<keyword!("struct")>(context)
    }
}

impl Parse for Fields {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        dbg!("fields start");

        if let Some(group) = parser.parse::<Option<Group<Braces>>>(context) {
            Self::Named(
                group
                    .into_parser()
                    .parse_trl_all::<_, punct!(",")>(context)
                    .into_iter()
                    .collect(),
            )
        } else if let Some(group) = parser.parse::<Option<Group<Parens>>>(context) {
            Self::Unnamed(
                group
                    .into_parser()
                    .parse_trl_all::<_, punct!(",")>(context)
                    .into_iter()
                    .collect(),
            )
        } else {
            context.push_error(SyntaxError::Expected(
                parser.peek_span(),
                "either `{ }` or `( )`",
            ));

            dbg!("fields end");

            Self::Unknown
        }
    }
}

impl Parse for NamedField {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        let vis = parser.parse(context);

        let ident = match parser.try_parse(context) {
            Ok(ok) => Ok(ok),
            Err(_) => {
                while parser.peek_next().is_some() && !parser.peek::<punct!(",")>(context) {
                    parser.next();
                }

                return Self {
                    vis,
                    ident: Err(()),
                    type_: Expr::Unknown(parser.peek_span()),
                    bounds: None,
                };
            }
        };

        let type_ = if let Some(_) = parser.parse::<Option<punct!("-")>>(context) {
            parser.parse(context)
        } else {
            context.push_error(SyntaxError::Expected(
                parser.peek_span(),
                "`ParamIdent-ParamType`",
            ));

            Expr::Unknown(parser.peek_span())
        };

        let bounds = if let Some(_) = parser.parse::<Option<punct!(":")>>(context) {
            Some(parser.parse(context))
        } else {
            None
        };

        Self {
            vis,
            ident,
            type_,
            bounds,
        }
    }
}

impl Detect for NamedField {
    fn detect(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> bool {
        parser.peek::<Ident>(context)
    }
}

impl Parse for UnnamedField {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        let vis = parser.parse(context);

        let type_ = parser.parse(context);

        let bounds = if let Some(_) = parser.parse::<Option<punct!(":")>>(context) {
            Some(parser.parse(context))
        } else {
            None
        };

        Self { vis, type_, bounds }
    }
}

impl Detect for UnnamedField {
    fn detect(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> bool {
        parser.peek::<Expr>(context)
    }
}
