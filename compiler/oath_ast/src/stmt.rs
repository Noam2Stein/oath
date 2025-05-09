use crate::*;

#[derive(Debug, Clone, OptionParse)]
#[desc = "a statement"]
pub enum Stmt {
    Let(LetStmt),
    Expr(Expr),
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a let statement"]
pub struct LetStmt {
    pub _keyword: keyword!("let"),
    pub name: Try<VarName>,
    pub init: Try<LetStmtInit>,
}
#[derive(Debug, Clone, OptionParse)]
#[desc = "a let statement"]
pub struct LetStmtInit {
    pub eq: punct!("="),
    pub init: Try<Expr>,
}
