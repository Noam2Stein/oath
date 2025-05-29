use super::*;

#[derive(Debug)]
pub enum TypeItem {}

#[derive(Debug)]
pub struct Mod {
    ident: Try<Ident>,
    types: Vec<Owned<TypeItem>>,
}

impl QueryType for TypeItem {
    type Ast = ;
}
