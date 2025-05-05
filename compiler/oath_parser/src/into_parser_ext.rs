use super::*;

#[allow(private_bounds)]
pub trait IntoParserExt<'src, 'ctx, 'parent>: Seal {
    fn into_parser(self) -> Parser<'src, 'ctx, 'parent>;
}
trait Seal {}

impl<'src, 'ctx, 'parent> Seal for Tokenizer<'src, 'ctx, 'parent> {}
impl<'src, 'ctx, 'parent> IntoParserExt<'src, 'ctx, 'parent> for Tokenizer<'src, 'ctx, 'parent> {
    fn into_parser(self) -> Parser<'src, 'ctx, 'parent> {
        Parser::new(self)
    }
}
