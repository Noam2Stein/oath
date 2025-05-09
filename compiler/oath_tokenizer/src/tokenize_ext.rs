use crate::*;

#[allow(private_bounds)]
pub trait TokenizeExt: Seal {
    fn tokenize<'src, 'ctx>(&'src self, context: ContextHandle<'ctx>) -> RootTokenizer<'src, 'ctx>;
}
trait Seal {}

impl Seal for SrcFile {}
impl TokenizeExt for SrcFile {
    fn tokenize<'src, 'ctx>(&'src self, context: ContextHandle<'ctx>) -> RootTokenizer<'src, 'ctx> {
        RootTokenizer::new(self, context)
    }
}
