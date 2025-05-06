use crate::*;

#[derive(Debug, Clone, Spanned, OptionParse)]
#[desc = "a statement"]
pub enum Stmt {
    Let(LetStmt),
    Expr(Expr),
}

#[derive(Debug, Clone, Spanned, OptionParse)]
#[desc = "a let statement"]
pub struct LetStmt {
    pub _keyword: keyword!("let"),
    #[option_spanned]
    pub name: VarName,
    #[option_spanned]
    pub init: Try<Expr>,
}
