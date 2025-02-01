use oath_diagnostics::DiagnosticsHandle;
use oath_parser::ParseExt;
use oath_tokenizer::TokenFile;

mod mod_;
mod mod_content;
mod mod_item;
mod path;
mod type_;
mod use_;
pub use mod_::*;
pub use mod_content::*;
pub use mod_item::*;
pub use path::*;
pub use type_::*;
pub use use_::*;

trait Seal {}

#[allow(private_bounds)]
pub trait ParseAstExt: Seal {
    fn parse_ast(self, diagnostics: DiagnosticsHandle) -> ModContent;
}

impl Seal for TokenFile {}
impl ParseAstExt for TokenFile {
    fn parse_ast(self, diagnostics: DiagnosticsHandle) -> ModContent {
        self.tokens.into_iter().peekable().parse(diagnostics)
    }
}
