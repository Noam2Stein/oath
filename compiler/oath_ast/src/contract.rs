use crate::*;

#[derive(Debug, Clone, Default, Desc)]
#[desc = "a contract"]
pub struct Contract {
    pub promise: Vec<ContractItem>,
    pub require: Vec<ContractItem>,
}

#[derive(Debug, Clone, Desc)]
#[desc = "a contract item"]
pub struct ContractItem {
    pub target: Expr,
    pub bounds: Expr,
}

impl Parse for Contract {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        let mut output = Self::default();

        loop {
            if parser
                .parse::<Option<keyword!("promise")>>(context)
                .is_some()
            {
                for promise in parser.parse_trl::<_, punct!(",")>(context).into_iter() {
                    output.promise.push(promise);
                }
            } else if parser
                .parse::<Option<keyword!("require")>>(context)
                .is_some()
            {
                for require in parser.parse_trl::<_, punct!(",")>(context).into_iter() {
                    output.require.push(require);
                }
            } else {
                break;
            }
        }

        output
    }
}

impl Parse for ContractItem {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        let target = Expr::parse_no_mhs(parser, context);

        match parser.try_parse::<punct!(":")>(context) {
            Ok(ok) => ok,
            Err(()) => {
                return Self {
                    target,
                    bounds: Expr::Unknown(parser.next_span()),
                }
            }
        };

        let bounds = parser.parse(context);

        Self { target, bounds }
    }
}
impl Peek for ContractItem {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Expr>(context)
    }
}
