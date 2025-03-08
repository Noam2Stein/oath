use crate::*;

#[derive(Debug)]
pub enum Item {
    Scope(ScopeId),
    Type(Type),
}
