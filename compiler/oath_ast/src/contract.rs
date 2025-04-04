use crate::*;

#[derive(Debug, Clone, Default)]
pub struct Contract {
    pub promise: Vec<ContractItem>,
    pub require: Vec<ContractItem>,
}

#[derive(Debug, Clone, Parse)]
pub struct ContractItem {
    pub target: Try<Expr>,
    pub bounds: Try<Bounds>,
}

impl Parse for Contract {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        let parser = &mut parser.until(|parser| <BracesOrSemi<()>>::detect(parser));

        let mut output = Self::default();

        loop {
            if let Some(_) = <Option<keyword!("promise")>>::parse(parser) {
                output
                    .promise
                    .append(&mut parser.parse_trl::<_, punct!(",")>());
            } else if let Some(_) = <Option<keyword!("require")>>::parse(parser) {
                output
                    .require
                    .append(&mut parser.parse_trl::<_, punct!(",")>());
            } else {
                break;
            }
        }

        output
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
