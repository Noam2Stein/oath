use super::*;

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct GenericValues {
    pub angles: InAngles<SeperatedMaybeTrailing<Type, Punct!(",")>>,
}
