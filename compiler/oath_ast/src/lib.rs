use splat_attribs::splat_attribs;
splat_attribs! {
    #[allow(unused_imports)]:

    use oath_context::*;
    use oath_parser::*;
    use oath_src::*;
    use oath_tokenizer::*;
}

mod block;
mod contract;
mod error;
mod expr;
mod fn_;
mod generic_args;
mod generic_params;
mod item;
mod mod_;
mod path;
mod struct_;
mod trait_;
mod type_;
mod vis;
pub use block::*;
pub use contract::*;
pub use error::*;
pub use expr::*;
pub use fn_::*;
pub use generic_args::*;
pub use generic_params::*;
pub use item::*;
pub use mod_::*;
pub use path::*;
pub use struct_::*;
pub use trait_::*;
pub use type_::*;
pub use vis::*;

trait Seal {}

pub type SyntaxTree = ModContent;

#[allow(private_bounds)]
pub trait ParseAstExt: Seal {
    fn parse_ast(self, context: ContextHandle) -> SyntaxTree;
}

impl Seal for TokenFile {}
impl ParseAstExt for TokenFile {
    fn parse_ast(self, context: ContextHandle) -> SyntaxTree {
        self.into_parser().parse(context).unwrap()
    }
}
