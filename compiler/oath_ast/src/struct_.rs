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
    Unnamed(Vec<PResult<Expr>>),
}

#[derive(Debug, Clone, Desc)]
#[desc = "a named fiend"]
pub struct NamedField {
    pub ident: Ident,
    pub bounds: PResult<Expr>,
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
                    .try_parse_trl_all::<_, punct!(",")>(context)
                    .into_iter()
                    .filter_map(Result::ok)
                    .collect(),
            )),
            DelimiterKind::Parens => Ok(Self::Unnamed(
                group
                    .into_parser()
                    .try_parse_trl_all::<_, punct!(",")>(context)
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

impl TryParse for NamedField {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> PResult<Self> {
        let ident = parser.try_parse(context)?;

        let bounds = if let Ok(_) = parser.try_parse::<punct!(":")>(context) {
            parser.try_parse(context)
        } else {
            Err(())
        };

        Ok(Self { ident, bounds })
    }
}

impl Peek for NamedField {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Ident>(context)
    }
}
