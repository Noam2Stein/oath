use std::{collections::HashMap, sync::Arc};

use crate::Item;

#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub parent: Option<Box<Scope>>,
    pub names: HashMap<String, Arc<Item>>,
}
