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
    use oath_tokenizer::*;
}

mod expr;
mod item;
mod param;
mod stmt;
mod var_name;
pub use expr::*;
pub use item::*;
pub use param::*;
pub use stmt::*;
pub use var_name::*;

pub type SyntaxTree = ModContent;

#[allow(private_bounds)]
pub trait ParseAstExt: Seal {
    fn parse_ast(self) -> SyntaxTree;
}
trait Seal {}

impl<Src: TokenSource> Seal for Tokenizer<Src> {}
impl<Src: TokenSource> ParseAstExt for Tokenizer<Src> {
    fn parse_ast(self) -> SyntaxTree {
        let mut parser = Parser(self);

        let mut output = SyntaxTree::parse_error();

        SyntaxTree::parse(&mut parser, &mut output);

        output
    }
}
