use oath_ast::SyntaxTree;
use oath_diagnostics::{DiagnosticsHandle, Error};
use oath_src::Spanned;

mod item;
mod item_attrib;
mod mod_;
mod struct_;
mod scope;
pub use item::*;
pub use struct_::*;
pub use item_attrib::*;
pub use mod_::*;
pub use scope::*;

pub trait NameResExt: Seal {
    fn name_res(self, diagnostics: DiagnosticsHandle) -> Scope;
}

impl NameResExt for SyntaxTree {
    fn name_res(self, diagnostics: DiagnosticsHandle) -> Scope {
        let mut attributes = Vec::new();
        let mut output = Scope::default();

        for segment in self.segments.0 {
            match segment {
                oath_ast::ModSegment::Con(segment) => attributes.push(ItemAttrib::Con(segment)),
                oath_ast::ModSegment::Pub(segment) => attributes.push(ItemAttrib::Pub(segment)),
                oath_ast::ModSegment::Raw(segmment) => attributes.push(ItemAttrib::Raw(segmment)),
                oath_ast::ModSegment::Struct(segment) => 
            }
        }

        output
    }
}
impl Seal for SyntaxTree {}

trait Seal {}
