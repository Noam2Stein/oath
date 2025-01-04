use std::any::Any;

use oath_diagnostics::DiagnosticsHandle;
use oath_tokenizer::TokenFile;

trait Seal {}

#[allow(private_bounds)]
pub trait TokenFileParseAstExt: Seal {
    fn parse_ast(self, diagnostics: &mut DiagnosticsHandle) -> impl Any;
}

impl Seal for TokenFile {}
impl TokenFileParseAstExt for TokenFile {
    fn parse_ast(self, diagnostics: &mut DiagnosticsHandle) -> impl Any {
        todo!()
    }
}
