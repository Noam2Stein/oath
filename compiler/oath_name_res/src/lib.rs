use oath_ast::SyntaxTree;
use oath_src::Spanned;

mod item;
mod mod_;
mod scope;
pub use item::*;
pub use mod_::*;
use oath_diagnostics::{DiagnosticsHandle, Error};
pub use scope::*;

pub trait NameResExt: Seal {
    fn name_res(self, diagnostics: DiagnosticsHandle) -> Scope;
}

impl NameResExt for SyntaxTree {
    fn name_res(self, diagnostics: DiagnosticsHandle) -> Scope {
        let mut output = Scope::default();

        for segment in self.segments.0 {
            match segment {
                oath_ast::ModSegment::Con(token) => {
                    diagnostics.push_error(Error::StaticMessage("unexpected token"), token.span())
                }
                oath_ast::ModSegment::Pub(token) => {
                    diagnostics.push_error(Error::StaticMessage("unexpected token"), token.span())
                }
            }
        }

        output
    }
}
impl Seal for SyntaxTree {}

trait Seal {}
