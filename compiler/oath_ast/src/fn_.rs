use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "a fn"]
pub struct Fn {
    pub vis: Vis,
    pub con: Option<keyword!("con")>,
    pub raw: Option<keyword!("raw")>,
    pub ident: Ident,
    pub generics: GenericParams,
    pub params: Vec<PResult<FnParam>>,
    pub output: Option<PResult<Expr>>,
    pub contract: Contract,
    pub block: BracesOrSemi<()>,
}

#[derive(Debug, Clone, Desc)]
#[desc = "a fn param"]
pub struct FnParam {
    pub mut_: Option<keyword!("mut")>,
    pub ident: Ident,
    pub bounds: Option<Expr>,
}

impl ItemParse for Fn {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
        target_kind: ItemKind,
    ) -> PResult<Self> {
        let vis = modifiers.take_vis();
        let con = modifiers.take_con();
        let raw = modifiers.take_raw();

        target_kind.expect_empty(context, Self::desc());

        let ident = parser.try_parse(context)?;
        let generics = parser.parse(context);
        let params = parser
            .try_parse::<Group<Parens>>(context)?
            .into_parser()
            .try_parse_trl_all::<_, punct!(",")>(context);

        let output = if let Some(_) = parser.parse::<Option<punct!("->")>>(context) {
            Some(parser.try_parse(context))
        } else {
            None
        };

        let contract = parser.parse(context);
        let block = parser.parse(context);

        Ok(Self {
            raw,
            vis,
            con,
            contract,
            generics,
            ident,
            params,
            output,
            block,
        })
    }
}

impl Peek for Fn {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<keyword!("fn")>(context)
    }
}

impl TryParse for FnParam {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> PResult<Self> {
        let mut_ = parser.parse(context);
        let ident = parser.try_parse(context)?;

        let bounds = if let Some(_) = parser.parse::<Option<punct!(":")>>(context) {
            parser.try_parse(context).ok()
        } else {
            None
        };

        Ok(Self {
            mut_,
            ident,
            bounds,
        })
    }
}

impl Peek for FnParam {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Ident>(context)
    }
}
