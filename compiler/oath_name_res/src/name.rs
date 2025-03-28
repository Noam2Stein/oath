use crate::*;

#[derive(Debug, Clone)]
pub enum DumbName {
    Type(DumbType, NamespaceId),
}

pub enum ResName {
    Type(ResType),
}
