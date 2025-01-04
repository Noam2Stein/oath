mod tokens;
use oath_diagnostics::DiagnosticsHandle;
pub use tokens::*;

mod raw_tokenizer;

use oath_src::SrcFile;

trait Seal {}

#[allow(private_bounds)]
pub trait SrcFileTokenizeExt: Seal {
    fn tokenize(&self, diagnostics: &mut DiagnosticsHandle) -> TokenFile;
}

impl Seal for SrcFile {}
impl SrcFileTokenizeExt for SrcFile {
    fn tokenize(&self, diagnostics: &mut DiagnosticsHandle) -> TokenFile {
        todo!()
    }
}
