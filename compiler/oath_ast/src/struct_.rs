use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "a struct"]
pub struct Struct {
    pub vis: Vis,
    pub ident: Ident,
    pub generics: GenericParams,
    pub contract: Contract,
    pub fields: PResult<Fields>,
}

#[derive(Debug, Clone, Desc)]
#[desc = "fields"]
pub enum Fields {
    Named(Vec<NamedField>),
    Unnamed(Vec<UnnamedField>),
}

#[derive(Debug, Clone, Desc)]
#[desc = "a named fiend"]
pub struct NamedField {
    pub vis: Vis,
    pub ident: PResult<Ident>,
    pub type_: PResult<Expr>,
    pub bounds: Option<Expr>,
}

#[derive(Debug, Clone, Desc)]
#[desc = "an unnamed fiend"]
pub struct UnnamedField {
    pub vis: Vis,
    pub type_: PResult<Expr>,
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
        let contract = parser.parse(context);
        let fields = parser.try_parse(context);

        Ok(Self {
            vis,
            ident,
            generics,
            contract,
            fields,
        })
    }
}

impl Peek for Struct {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<keyword!("struct")>(context)
    }
}

impl TryParse for Fields {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> PResult<Self> {
        let group = parser.try_parse::<Group>(context)?;
        match group.delimiters.kind {
            DelimiterKind::Braces => Ok(Self::Named(
                group
                    .into_parser()
                    .parse_trl_all::<_, punct!(",")>(context)
                    .into_iter()
                    .collect(),
            )),
            DelimiterKind::Parens => Ok(Self::Unnamed(
                group
                    .into_parser()
                    .parse_trl_all::<_, punct!(",")>(context)
                    .into_iter()
                    .collect(),
            )),
            _ => {
                context.push_error(SyntaxError::Expected(group.span(), "either `{ }` or `( )`"));
                Err(())
            }
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
                    type_: Err(()),
                    bounds: None,
                };
            }
        };

        let type_ = if let Some(_) = parser.parse::<Option<punct!("-")>>(context) {
            parser.try_parse(context)
        } else {
            context.push_error(SyntaxError::Expected(
                parser.next_span(),
                "`ParamIdent-ParamType`",
            ));

            Err(())
        };

        if type_.is_err() {
            while parser.peek_next().is_some()
                && !parser.peek::<punct!(",")>(context)
                && !parser.peek::<punct!(":")>(context)
            {
                parser.next();
            }
        }

        let bounds = if let Some(_) = parser.parse::<Option<punct!(":")>>(context) {
            parser.try_parse(context).ok()
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

impl Peek for NamedField {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Ident>(context)
    }
}

impl Parse for UnnamedField {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        let vis = parser.parse(context);

        let type_ = parser.try_parse(context);

        if type_.is_err() {
            while parser.peek_next().is_some()
                && !parser.peek::<punct!(",")>(context)
                && !parser.peek::<punct!(":")>(context)
            {
                parser.next();
            }
        }

        let bounds = if let Some(_) = parser.parse::<Option<punct!(":")>>(context) {
            parser.try_parse(context).ok()
        } else {
            None
        };

        Self { vis, type_, bounds }
    }
}

impl Peek for UnnamedField {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Expr>(context)
    }
}
