use super::*;

#[derive(Debug, Clone, OptionParse)]
#[desc = "`{ }`"]
#[framed]
pub struct Block {
    pub delims: delims!("{ }"),
    pub stmts: List<Stmt>,
}

#[derive(Debug, Clone, OptionParse)]
#[desc = "a statement"]
pub enum Stmt {
    Let {
        keyword: keyword!("let"),
        name: Try<VarName>,
        init: Try<VarInit>,
    },
    Eval(keyword!("eval"), Try<Expr>),
    Return(keyword!("eval"), Try<Expr>),
    Break(keyword!("break"), Try<Expr>),
    Continue(keyword!("continue"), Try<Expr>),
    Expr(Expr),
}
