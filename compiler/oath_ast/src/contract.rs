use crate::*;

#[derive(Debug, Clone, Default)]
pub struct Contract {
    pub segments: Vec<ContractSegment>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "either `require` or `promise`"]
pub struct ContractSegment {
    pub kind: ContractSegmentKind,
    pub items: Vec<ContractItem>,
}

#[derive(Debug, Clone, Copy, Spanned, OptionParse)]
#[desc = "either `require` or `promise`"]
pub enum ContractSegmentKind {
    Require(keyword!("require")),
    Promise(keyword!("promise")),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "contract item"]
pub struct ContractItem {
    pub target: Expr,
    pub _colon: Try<punct!(":")>,
    pub bounds: Try<Bounds>,
}

impl Parse for Contract {
    fn parse(parser: &mut Parser<impl ParserIterator>, output: &mut Self) -> ParseExit {
        let parser = &mut parser.until(BracesOrSemi::<()>::detect);

        loop {
            if let Some(_) = {
                let mut require = None;
                <Option<keyword!("require")>>::parse(parser, &mut require);
                require
            } {
                parser.parse_trl::<_, punct!(",")>(&mut output.require);
            } else if let Some(_) = {
                let mut promise = None;
                <Option<keyword!("promise")>>::parse(parser, &mut promise);
                promise
            } {
                parser.parse_trl::<_, punct!(",")>(&mut output.promise);
            } else {
                break ParseExit::Complete;
            }
        }
    }

    fn parse_error() -> Self {
        Self {
            promise: Vec::new(),
            require: Vec::new(),
        }
    }
}

impl Contract {
    pub fn is_empty(&self) -> bool {
        self.promise.len() == 0 && self.require.len() == 0
    }
    pub fn is_not_empty(&self) -> bool {
        self.promise.len() > 0 || self.require.len() > 0
    }
}
