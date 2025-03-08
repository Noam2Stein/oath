use std::collections::HashMap;

use oath_context::{ContextHandle, StrId};

use crate::*;

#[derive(Debug)]
pub struct Scope {
    items: HashMap<StrId, Item>,
    unreachable: Vec<Item>,
}

#[derive(Debug)]
pub struct ScopeContext {
    scopes: Vec<Scope>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeId(usize);

impl Scope {
    pub fn new(items: HashMap<StrId, Item>, unreachable: Vec<Item>) -> Self {
        Self { items, unreachable }
    }

    pub fn item(&self, str: StrId) -> Option<&Item> {
        self.items.get(&str)
    }
    pub fn item_mut(&mut self, str: StrId) -> Option<&mut Item> {
        self.items.get_mut(&str)
    }
}

impl ScopeContext {
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    pub fn push_scope(&mut self, scope: Scope) -> ScopeId {
        let id = ScopeId(self.scopes.len());
        self.scopes.push(scope);

        id
    }

    pub fn scope(&self, id: ScopeId) -> &Scope {
        &self.scopes[id.0]
    }

    pub fn scope_mut(&mut self, id: ScopeId) -> &mut Scope {
        &mut self.scopes[id.0]
    }

    pub fn build_scope<I>(
        &mut self,
        item_input: impl Iterator<Item = I>,
        map_item: impl Fn(I, &mut Self) -> (StrId, Item),
        context: ContextHandle,
    ) -> ScopeId {
    }
}
