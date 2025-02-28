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
                for promise in parser
                    .try_parse_trl::<_, punct!(",")>(context)
                    .into_iter()
                    .filter_map(Result::ok)
                {
                    output.promise.push(promise);
                }
            } else if parser
                .parse::<Option<keyword!("require")>>(context)
                .is_some()
            {
                for require in parser
                    .try_parse_trl::<_, punct!(",")>(context)
                    .into_iter()
                    .filter_map(Result::ok)
                {
                    output.require.push(require);
                }
            } else {
                break;
            }
        }

        output
    }
}

impl TryParse for ContractItem {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        let target = parser.try_parse(context)?;

        parser.try_parse::<punct!(":")>(context)?;

        let bounds = parser.try_parse(context)?;

        Ok(Self { target, bounds })
    }
}
impl Peek for ContractItem {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Expr>(context)
    }
}
