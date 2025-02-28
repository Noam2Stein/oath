use crate::*;

#[derive(Debug, Clone, Spanned)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub ditch: bool,
    #[span]
    span: Span,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Var(VarStmt),
}

#[derive(Debug, Clone, Spanned)]
pub struct VarStmt {
    pub ident: Ident,
    pub ty: Option<Expr>,
    pub init: Option<Expr>,
    #[span]
    span: Span,
}
