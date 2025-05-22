use std::fmt::Debug;

use oath_highlighting::*;
use oath_parser::*;
use oath_src::*;
use oath_tokenizer::*;
use oath_tokens::*;

mod attr;
mod block;
mod expr;
mod item;
pub use attr::*;
pub use block::*;
pub use expr::*;
pub use item::*;

pub type SyntaxTree = Repeated<Item>;

#[allow(private_bounds)]
pub trait ParseAstExt: Seal {
    fn parse_ast(self) -> SyntaxTree;
}
trait Seal {}

impl<T: Tokenizer> Seal for T {}
impl<T: Tokenizer> ParseAstExt for T {
    fn parse_ast(self) -> SyntaxTree {
        let mut parser = Parser(self);

        let mut output = SyntaxTree::parse_error();

        SyntaxTree::parse(&mut parser, &mut output);

        output
    }
}
