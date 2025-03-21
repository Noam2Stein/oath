use crate::*;

#[derive(Debug, Clone, Desc)]
#[desc = "a fn"]
pub struct Fn {
    pub vis: Vis,
    pub con: Option<keyword!("con")>,
    pub raw: Option<keyword!("raw")>,
    pub ident: Ident,
    pub generics: Option<GenericParams>,
    pub params: Vec<FnParam>,
    pub output: Option<Expr>,
    pub contract: Contract,
    pub block: BracesOrSemi<()>,
}

#[derive(Debug, Clone, Desc)]
#[desc = "a fn param"]
pub struct FnParam {
    pub mut_: Option<keyword!("mut")>,
    pub ident: PResult<Ident>,
    pub type_: Expr,
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
            .parse_trl_all::<_, punct!(",")>(context);

        let output = if let Some(_) = parser.parse::<Option<punct!("->")>>(context) {
            Some(parser.parse(context))
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

impl Parse for FnParam {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        let mut_ = parser.parse(context);

        let ident = match parser.try_parse(context) {
            Ok(ok) => Ok(ok),
            Err(()) => {
                parser.skip_until(|parser| parser.peek::<punct!(",")>(context));
                return Self {
                    mut_,
                    ident: Err(()),
                    type_: Expr::Unknown(parser.next_span()),
                    bounds: None,
                };
            }
        };

        let type_ = if let Some(_) = parser.parse::<Option<punct!("-")>>(context) {
            parser.parse(context)
        } else {
            context.push_error(SyntaxError::Expected(
                parser.next_span(),
                "`param_ident-Param_Type`",
            ));

            Expr::Unknown(parser.next_span())
        };

        let bounds = parser
            .parse::<Option<punct!(":")>>(context)
            .map(|_| parser.parse(context));

        Self {
            mut_,
            ident,
            type_,
            bounds,
        }
    }
}

impl Peek for FnParam {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Ident>(context)
    }
}
