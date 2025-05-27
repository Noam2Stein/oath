use super::*;

#[derive(Debug)]
pub enum TypeItem {}

#[derive(Debug)]
pub enum ItemId {
    Type(Id<TypeItem>),
}

impl QueryType for TypeItem {}
