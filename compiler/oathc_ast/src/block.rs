use super::*;

#[derive(Debug, OptionParse)]
#[desc = "`{ }`"]
#[framed]
pub struct Block {
    pub delims: delims!("{ }"),
    pub stmts: Repeated<Stmt>,
    pub leftovers: Leftovers,
}

#[derive(Debug, OptionParse)]
#[desc = "a statement"]
pub enum Stmt {
    Item(Item),
    Let {
        keyword: keyword!("let"),
        name: Try<VarName>,
        init: Try<VarInit>,
        semi: Try<punct!(";")>,
    },
    Eval(keyword!("eval"), Try<Expr>, Try<punct!(";")>),
    Return(keyword!("return"), Option<Expr>, Try<punct!(";")>),
    Break(keyword!("break"), Option<Expr>, Try<punct!(";")>),
    Continue(keyword!("continue"), Option<Expr>, Try<punct!(";")>),
    Expr(Expr, Option<SetStmt>, Try<punct!(";")>),
}

#[derive(Debug, OptionParse)]
#[desc = "`= ...`"]
pub struct SetStmt {
    pub eq: punct!("="),
    pub value: Try<Expr>,
}
