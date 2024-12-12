use super::*;

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct Block {
    pub braces: InBraces<LineTerminated<Stmt>>,
}

#[derive(Debug, Clone, Hash, Parse)]
pub enum Stmt {
    Var(Var),
    Expr(Expr),
}
