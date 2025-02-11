use crate::*;

#[derive(Default)]
pub struct Contract {
    pub promise: Vec<ContractItem>,
    pub require: Vec<ContractItem>,
}

pub struct ContractItem {
    pub target: Expr,
    pub bounds: Trait,
}

impl Parse for Contract {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        let mut output = Self::default();

        loop {
            if parser
                .parse_option::<keyword!("promise")>(context)
                .is_some()
            {
                if let Ok(mut promise) = parser.parse_sep::<_, punct!(","), true, true>(context) {
                    output.promise.append(&mut promise);
                }
            } else if parser
                .parse_option::<keyword!("require")>(context)
                .is_some()
            {
                if let Ok(mut require) = parser.parse_sep::<_, punct!(","), true, true>(context) {
                    output.require.append(&mut require);
                }
            } else {
                break;
            }
        }

        Ok(output)
    }
}

impl Parse for ContractItem {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        let target = parser.parse(context)?;

        parser.parse::<punct!(":")>(context)?;

        let bounds = parser.parse(context)?;

        Ok(Self { bounds, target })
    }
}
impl Peek for ContractItem {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Expr>(context)
    }
}
