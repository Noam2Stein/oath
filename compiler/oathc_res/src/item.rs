use super::*;

#[derive(Debug)]
pub struct Mod {
    types: Vec<Id<TypeItem>>,
}

#[derive(Debug)]
pub enum TypeItem {}

impl ResType for Mod {
    type Src = ;
}

impl ResType for TypeItem {
    type Src = ;
}
