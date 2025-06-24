use std::fmt::Debug;

use oathc_diagnostics::*;
use oathc_highlighting::*;
use oathc_interner::*;
use oathc_parser::*;
use oathc_span::*;
use oathc_tokenizer::*;
use oathc_tokens::*;

mod attr;
mod block;
mod contract;
mod control_flow;
mod expr;
mod generics;
mod item;
mod param;
mod type_;
pub use attr::*;
pub use block::*;
pub use contract::*;
pub use control_flow::*;
pub use expr::*;
pub use generics::*;
pub use item::*;
pub use param::*;
pub use type_::*;

#[derive(Debug, Default)]
pub struct SyntaxTree {
    pub items: Vec<Item>,
    pub leftovers: Leftovers,
}

#[allow(private_bounds)]
pub trait ParseAstExt: Seal {
    fn parse_ast(self) -> SyntaxTree;
}
trait Seal {}

impl<T: Tokenizer> Seal for T {}
impl<T: Tokenizer> ParseAstExt for T {
    fn parse_ast(self) -> SyntaxTree {
        let mut parser = self;

        let mut items = Repeated::parse_error();
        Parse::parse(&mut parser, &mut items);

        let leftovers = Leftovers::collect(&mut parser);

        SyntaxTree {
            items: items.into(),
            leftovers,
        }
    }
}
