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
    pub block: Option<()>,
}

#[derive(Debug, Clone, Desc)]
#[desc = "a fn param"]
pub struct FnParam {
    pub mut_: Option<keyword!("mut")>,
    pub ident: Ident,
    pub type_: PResult<Expr>,
    pub bounds: Option<Expr>,
}

impl ItemType for Fn {
    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
    ) -> Result<Self, ()> {
        let vis = modifiers.take_vis();
        let con = modifiers.take_con();
        let raw = modifiers.take_raw();

        parser.try_parse::<keyword!("fn")>(context)?;

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

        let block = parser.parse::<Option<Group<Braces>>>(context).map(|_| ());

        if block.is_none() {
            let _ = parser.try_parse::<punct!(";")>(context);
        }

        Ok(Self {
            block,
            con,
            contract,
            generics,
            ident,
            output,
            params,
            raw,
            vis,
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

        match parser.try_parse::<punct!(":")>(context) {
            Ok(value) => value,
            Err(()) => {
                return Ok(Self {
                    mut_,
                    ident,
                    type_: Err(()),
                    bounds: None,
                });
            }
        };

        let type_ = match parser.try_parse(context) {
            Ok(value) => Ok(value),
            Err(()) => {
                return Ok(Self {
                    mut_,
                    ident,
                    type_: Err(()),
                    bounds: None,
                })
            }
        };

        let bounds = if let Some(_) = parser.parse::<Option<punct!(":")>>(context) {
            match parser.try_parse(context) {
                Ok(value) => Some(value),
                Err(()) => None,
            }
        } else {
            None
        };

        Ok(Self {
            mut_,
            ident,
            type_,
            bounds,
        })
    }
}

impl Peek for FnParam {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Ident>(context)
    }
}
