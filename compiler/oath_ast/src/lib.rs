use std::fmt::Debug;

use splat_attribs::splat_attribs;
splat_attribs! {
    #[allow(unused_imports)]:

    use oath_context::*;
    use oath_diagnostics::*;
    use oath_highlighting::*;
    use oath_parser::*;
    use oath_src::*;
    use oath_tokens::*;
}

mod block;
mod bounds;
mod braces_or_semi;
mod contract;
mod expr;
mod func;
mod generic_args;
mod generic_params;
mod impl_;
mod item;
mod mod_;
mod struct_;
mod sys;
mod trait_;
mod var_name;
mod vis;
pub use block::*;
pub use bounds::*;
pub use braces_or_semi::*;
pub use contract::*;
pub use expr::*;
pub use func::*;
pub use generic_args::*;
pub use generic_params::*;
pub use impl_::*;
pub use item::*;
pub use mod_::*;
pub use struct_::*;
pub use sys::*;
pub use trait_::*;
pub use var_name::*;
pub use vis::*;

pub type SyntaxTree = ModContent;

#[allow(private_bounds)]
pub trait ParseAstExt: Seal {
    fn parse_ast(self, context: ContextHandle) -> SyntaxTree;
}
trait Seal {}

impl Seal for TokenFile {}
impl ParseAstExt for TokenFile {
    fn parse_ast(self, context: ContextHandle) -> SyntaxTree {
        let mut parser = self.into_parser(context);
        let mut output = SyntaxTree::parse_error();

        SyntaxTree::parse(&mut parser, &mut output);

        output
    }
}
