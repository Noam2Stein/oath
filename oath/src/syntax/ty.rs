use super::*;

#[derive(Debug, Clone, Hash, Parse)]
pub enum Type {
    Angles(InAngles<Box<Type>>),
    Path(Path),
    Tuple(InParens<TerminatedMaybeTrailing<Box<Type>, Punct!(",")>>),
    Dyn(TypeDyn),
    #[error]
    Error(Span),
}
#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct TypeDyn {
    pub dyn_token: Keyword!("dyn"),
    pub bounds: Bounds,
}
