use super::*;

#[derive(Debug, Parse)]
pub struct Contract {
    #[parse_as(Repeated<ContractSegment>)]
    pub segments: Vec<ContractSegment>,
}

#[derive(Debug, OptionParse)]
#[desc = "a contract segment"]
pub enum ContractSegment {
    Require(Require),
    Promise(Promise),
}

#[derive(Debug, OptionParse)]
#[desc = "`promise`"]
pub struct Require {
    pub keyword: keyword!("require"),
    pub items: Try<Array>,
}

#[derive(Debug, OptionParse)]
#[desc = "`promise`"]
pub struct Promise {
    pub keyword: keyword!("promise"),
    pub items: Try<Array>,
}
