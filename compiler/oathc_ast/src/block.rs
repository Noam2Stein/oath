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
    Return(keyword!("eval"), Try<Expr>, Try<punct!(";")>),
    Break(keyword!("break"), Try<Expr>, Try<punct!(";")>),
    Continue(keyword!("continue"), Try<Expr>, Try<punct!(";")>),
    Expr(Expr, Try<punct!(";")>),
}
