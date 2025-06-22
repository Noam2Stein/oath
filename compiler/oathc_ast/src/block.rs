use derive_more::Debug;

use super::*;

#[derive(Debug, Spanned, OptionParse)]
#[desc = "`{ }`"]
#[framed]
pub struct Block {
    pub frame: Frame<delims!("{ }")>,
    #[parse_as(Repeated<_>)]
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, OptionParse)]
#[desc = "a statement"]
pub enum Stmt {
    Item(Item),
    Let(LetStmt),
    Eval(ControlStmt<keyword!("eval")>),
    Return(ControlStmt<keyword!("return")>),
    Break(ControlStmt<keyword!("break")>),
    Continue(ControlStmt<keyword!("continue")>),
    Expr(ExprStmt),
}

#[derive(Debug, OptionParse)]
#[desc = "a statement"]
pub struct LetStmt {
    pub keyword: keyword!("let"),
    #[highlight(HighlightColor::Cyan)]
    pub name: Try<Param>,
    pub value: Option<Assign>,
    pub semi: Try<punct!(";")>,
}

#[derive(Debug, OptionParse)]
#[desc = "a statement"]
pub struct ExprStmt {
    pub expr: Expr,
    pub set: Option<Assign>,
    pub semi: Try<punct!(";")>,
}

#[derive(Debug, OptionParse)]
#[desc = "a statement"]
pub struct ControlStmt<K: OptionParse> {
    pub keyword: K,
    pub value: Try<Expr>,
    pub semi: Try<punct!(";")>,
}
