use oath_tokens::Ident;

use super::*;

#[derive(OptionParse)]
#[desc = "a parameter"]
pub struct Param {
    pub name: Ident,
    pub type_: Option<Expr>,
    pub bounds: Option<Bounds>,
}

#[derive(OptionParse)]
#[desc = "an unnamed parameter"]
pub struct UnnamedParam {
    pub type_: Expr,
    pub bounds: Option<Bounds>,
}

#[derive(Debug, Clone, OptionSpanned, OptionParse)]
#[desc = "`: ...`"]
pub struct Bounds {
    _colon: Discard<punct!(":")>,
    #[span]
    #[option_spanned]
    pub expr: Try<Expr>,
}
