use crate::*;

#[allow(private_bounds)]
pub trait TokenizeExt: Seal {
    fn tokenize<'ctx>(&'ctx self, context: &'ctx mut ParseContext) -> RootTokenizer<'ctx>;
}
trait Seal {}

impl Seal for str {}
impl TokenizeExt for str {
    fn tokenize<'ctx>(&'ctx self, context: &'ctx mut ParseContext) -> RootTokenizer<'ctx> {
        RootTokenizer::new(self, context)
    }
}
