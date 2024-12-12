use super::*;

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct Path {
    pub segments: SeperatedNotTrailing<PathSegment, Punct!("::")>,
}

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct PathSegment {
    pub ident: Ident,
    pub generics: Option<Box<GenericValues>>,
}
