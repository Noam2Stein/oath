use oath_tokens::Ident;

use super::*;

#[derive(OptionParse)]
#[desc = "a parameter"]
pub struct Param {
    pub name: Ident,
    pub type_: Option<Expr>,
    pub bounds: Option<Bounds>,
}
