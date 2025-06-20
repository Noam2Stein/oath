use oathc_ast::SyntaxTree;
use oathc_diagnostics::*;
use oathc_parser::Leftovers;
use oathc_span::*;
use oathc_tokens::*;

mod item;
//mod namespace;
pub use item::*;

pub trait ResolveExt {
    fn resolve(self, diagnostics: &Diagnostics) -> Mod;
}
impl ResolveExt for SyntaxTree {
    fn resolve(self, diagnostics: &Diagnostics) -> Mod {
        Mod {
            items: self
                .items
                .values
                .into_iter()
                .map(|item| Item::from_ast(item, diagnostics))
                .collect(),
            leftovers: self.leftovers,
        }
    }
}
