use oath_diagnostics::DiagnosticsHandle;
use oath_parser::IntoParser;
use oath_tokenizer::TokenFile;

mod block;
mod fn_;
mod generic_params;
mod mod_;
mod mod_content;
mod path;
mod pub_;
mod struct_;
mod use_;
pub use block::*;
pub use fn_::*;
pub use generic_params::*;
pub use mod_::*;
pub use mod_content::*;
pub use path::*;
pub use pub_::*;
pub use struct_::*;
pub use use_::*;

trait Seal {}

pub type SyntaxTree = ModContent;

#[allow(private_bounds)]
pub trait ParseAstExt: Seal {
    fn parse_ast(self, diagnostics: DiagnosticsHandle) -> SyntaxTree;
}

impl Seal for TokenFile {}
impl ParseAstExt for TokenFile {
    fn parse_ast(self, diagnostics: DiagnosticsHandle) -> ModContent {
        self.into_parser().parse(diagnostics)
    }
}
