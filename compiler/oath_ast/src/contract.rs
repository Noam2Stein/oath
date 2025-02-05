use oath_parser::{Parse, Peek, Trl};
use oath_tokenizer::{keyword, punct, KeywordType};

use crate::{Expr, TraitBounds};

#[derive(Parse)]
pub struct Contract {
    pub promise: Option<Where<keyword!("promise")>>,
    pub require: Option<Where<keyword!("require")>>,
}

#[derive(Parse, Peek)]
pub struct Where<K: KeywordType + Parse + Peek> {
    pub keyword: K,
    pub items: Trl<ContractItem, punct!(",")>,
}

#[derive(Parse, Peek)]
pub struct ContractItem {
    pub target: Expr,
    pub sep: punct!(":"),
    pub bounds: TraitBounds,
}
