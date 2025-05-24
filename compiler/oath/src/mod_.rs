use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModPath {
    segments: Vec<StrId>,
}

impl ModPath {
    pub fn new(segments: impl IntoIterator<Item = StrId>) -> Self {
        Self {
            segments: segments.into_iter().collect(),
        }
    }
    pub fn root() -> Self {
        Self { segments: Vec::new() }
    }

    pub fn chain(&self, inner: impl IntoIterator<Item = StrId>) -> Self {
        Self {
            segments: self.segments.iter().cloned().chain(inner).collect(),
        }
    }
    pub fn child(&self, name: StrId) -> Self {
        self.chain(std::iter::once(name))
    }
}
