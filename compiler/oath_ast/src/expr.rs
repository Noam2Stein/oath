use oath_diagnostics::Desc;
use oath_parser::{Garbage, Parse, Peek};
use oath_tokenizer::{punct, Literal};

use crate::Path;

#[derive(Parse, Peek, Desc)]
#[desc("an expr")]
pub enum Expr {
    Path(Path),
    Literal(Literal),
    Neg(NegExpr),
    Not(NotExpr),
    Deref(DerefExpr),
    #[dont_peek]
    Garbage(Garbage<Self>),
}

#[derive(Parse, Peek)]
pub struct NegExpr {
    pub neg: punct!("-"),
    pub expr: Box<Expr>,
}

#[derive(Parse, Peek)]
pub struct NotExpr {
    pub not: punct!("!"),
    pub expr: Box<Expr>,
}

#[derive(Parse, Peek)]
pub struct DerefExpr {
    pub deref: punct!("*"),
    pub expr: Box<Expr>,
}
