use crate::*;

#[derive(Debug, Clone, OptionParse)]
#[desc = "a statement"]
pub enum Stmt {
    Let(LetStmt),
    Eval(keyword!("eval"), Try<Expr>),
    Return(keyword!("eval"), Try<Expr>),
    Break(keyword!("break"), Try<Expr>),
    Continue(keyword!("continue"), Try<Expr>),
    Expr(Expr),
}

// LET

#[derive(Debug, Clone, OptionParse)]
#[desc = "a let statement"]
pub struct LetStmt {
    pub keyword: keyword!("let"),
    pub name: Try<VarName>,
    pub init: Try<LetStmtInit>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "`= ...`"]
pub struct LetStmtInit {
    pub eq: punct!("="),
    pub init: Try<Expr>,
}
