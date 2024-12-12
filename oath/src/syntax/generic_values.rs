use super::*;

#[derive(Debug, Clone, Hash, Parse, Peek)]
pub struct GenericValues {
    pub angles: InAngles<TerminatedMaybeTrailing<Type, Punct!(",")>>,
}
