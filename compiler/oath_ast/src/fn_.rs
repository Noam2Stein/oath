use crate::*;

pub struct Fn {
    pub vis: Vis,
    pub con: Option<keyword!("con")>,
    pub raw: Option<keyword!("raw")>,
    pub ident: Ident,
    pub generics: GenericParams,
    pub params: Vec<FnParam>,
    pub output: Result<Type, ()>,
    pub contract: Contract,
    pub block: Group<Braces>,
}

pub struct FnParam {
    pub mut_: Option<keyword!("mut")>,
    pub ident: Ident,
    pub type_: Result<Type, ()>,
    pub bounds: Option<Trait>,
}

impl ItemType for Fn {
    const DESC: &str = "a fn";

    fn item_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
        modifiers: &mut ItemModifiers,
    ) -> Result<Self, ()> {
        let vis = modifiers.take_vis();
        let con = modifiers.take_con();
        let raw = modifiers.take_raw();

        parser.parse::<keyword!("fn")>(context)?;

        let ident = parser.parse(context)?;
        let generics = parser.parse(context)?;
        let params = parser
            .parse::<Group<Parens>>(context)?
            .into_parser()
            .parse_sep_all::<_, punct!(","), false, true>(context)?;

        let output = if let Some(_) = parser.parse::<Option<punct!("->")>>(context)? {
            parser.parse(context)
        } else {
            Ok(Type::Tuple(Vec::new()))
        };

        let contract = parser.parse(context)?;

        let block = parser.parse(context)?;

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

    fn item_peek(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> bool {
        parser.peek::<keyword!("fn")>(context)
    }
}

impl Parse for FnParam {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        let mut_ = parser.parse(context)?;
        let ident = parser.parse(context)?;

        match parser.parse::<punct!(":")>(context) {
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

        let type_ = match parser.parse::<Type>(context) {
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

        let bounds = if let Some(_) = parser.parse::<Option<punct!(":")>>(context)? {
            match parser.parse(context) {
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
