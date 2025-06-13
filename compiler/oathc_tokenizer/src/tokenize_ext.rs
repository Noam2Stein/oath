use super::*;

#[allow(private_bounds)]
pub trait TokenizeExt: Seal {
    fn tokenize<'ctx>(
        &'ctx self,
        path: FileId,
        interner: &'ctx Interner,
        diagnostics: &'ctx Diagnostics,
        highlights: &'ctx mut Vec<Highlight>,
    ) -> RootTokenizer<'ctx>;
}
trait Seal {}

impl Seal for str {}
impl TokenizeExt for str {
    fn tokenize<'ctx>(
        &'ctx self,
        file: FileId,
        interner: &'ctx Interner,
        diagnostics: &'ctx Diagnostics,
        highlights: &'ctx mut Vec<Highlight>,
    ) -> RootTokenizer<'ctx> {
        RootTokenizer::new(self, file, interner, diagnostics, highlights)
    }
}
