use crate::*;

#[derive(Debug, Clone, OptionSpanned, OptionParse)]
#[desc = "`: ...`"]
pub struct Bounds {
    _colon: punct!(":"),
    #[option_spanned]
    pub expr: Try<Expr>,
}
