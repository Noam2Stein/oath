use super::*;

#[derive(Debug, Clone, Hash, Parse)]
pub enum Expr {
    Literal(Literal),
    Ident(Ident),
    Block(Box<Block>),
    Tuple(Box<InParens<TerminatedMaybeTrailing<Expr, Punct!(",")>>>),
    #[error]
    Error(Span),
}
