use super::*;

#[derive(Debug)]
pub struct Mod {
    pub items: Vec<ItemId>,
}

impl QueryType for Mod {}
