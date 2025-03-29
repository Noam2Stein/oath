use std::collections::HashMap;

use oath_context::{ContextHandle, Error, StrId};
use oath_parser::Try;
use oath_src::Spanned;
use oath_tokenizer::Ident;

use crate::*;

pub struct Namespace {
    map: HashMap<StrId, NameId>,
}

impl Namespace {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn push_name_id(&mut self, ident: Ident, name_id: NameId, context: ContextHandle) {
        if self.map.contains_key(&ident.str_id) {
            context.push_error(Error::new(
                format!(
                    "`{}` already exists in this context",
                    context.unintern(ident.str_id)
                ),
                ident.span(),
            ));
        } else {
            self.map.insert(ident.str_id, name_id);
        }
    }

    pub fn name_id(&self, str_id: StrId) -> Option<NameId> {
        self.map.get(&str_id).map(|name_id| *name_id)
    }

    pub fn try_name_id(&self, ident: Ident, context: ContextHandle) -> Try<NameId> {
        match self.name_id(ident.str_id) {
            Some(name_id) => Try::Success(name_id),
            None => {
                context.push_error(Error::new(
                    format!(
                        "`{}` doesn't exist in this context",
                        context.unintern(ident.str_id)
                    ),
                    ident.span(),
                ));

                Try::Failure
            }
        }
    }
}
