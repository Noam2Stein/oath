use super::*;

#[derive(Debug, Clone, Hash, Parse)]
pub struct Bounds {
    pub bounds: SeperatedMaybeTrailing<Bound, Punct!("+")>,
}

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub enum Bound {
    Path(Path),
    Not(BoundNot),
    #[error]
    Error(Span),
}

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct BoundNot {
    pub sep: Punct!("!"),
    pub path: Path,
}
