use std::sync::Arc;

use crate::*;

#[allow(private_bounds)]
pub trait TokenizeExt: Seal {
    fn tokenize<'src>(&'src self, context: Arc<Context>) -> RootTokenizer<'src>;
}
trait Seal {}

impl Seal for SrcFile {}
impl TokenizeExt for SrcFile {
    fn tokenize<'src>(&'src self, context: Arc<Context>) -> RootTokenizer<'src> {
        RootTokenizer::new(self, context)
    }
}
