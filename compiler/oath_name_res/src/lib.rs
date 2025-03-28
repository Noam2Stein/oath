use std::any::Any;

use oath_context::ContextHandle;

mod name;
mod name_context;
mod namespace;
mod ty;
pub use name::*;
pub use name_context::*;
pub use namespace::*;
use oath_parser::Try;
pub use ty::*;

pub trait NameResExt {
    fn name_res(self, context: ContextHandle) -> impl Any;
}

impl NameResExt for oath_ast::SyntaxTree {
    fn name_res(self, context: ContextHandle) -> impl Any {
        let mut name_context = DumbNameContext::new();

        let mod_namespace = name_context.push_namespace(Namespace::new());

        for item in self.items {
            match item {
                oath_ast::Item::Struct(item) => {
                    let ident = item.ident;
                    let name_id = name_context
                        .push_name(DumbName::Type(DumbType::Struct(item), mod_namespace));

                    if let Try::Success(ident) = ident {
                        name_context
                            .namespace_mut(mod_namespace)
                            .insert(ident.str_id, name_id);
                    }
                }
                _ => {}
            }
        }
    }
}
