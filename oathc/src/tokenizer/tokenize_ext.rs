use std::path::Path;

use super::*;

#[allow(private_bounds)]
pub trait TokenizeExt: Seal {
    fn tokenize<'ctx>(
        &'ctx self,
        path: &'ctx Path,
        interner: &'ctx Interner,
        diagnostics: &'ctx Diagnostics,
        highlights: &'ctx mut Vec<HighlightInfo>,
    ) -> RootTokenizer<'ctx>;
}
trait Seal {}

impl Seal for str {}
impl TokenizeExt for str {
    fn tokenize<'ctx>(
        &'ctx self,
        path: &'ctx Path,
        interner: &'ctx Interner,
        diagnostics: &'ctx Diagnostics,
        highlights: &'ctx mut Vec<HighlightInfo>,
    ) -> RootTokenizer<'ctx> {
        RootTokenizer::new(self, path, interner, diagnostics, highlights)
    }
}
