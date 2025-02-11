mod parse_box;
mod parse_tokens;

mod into_parser;
mod parser;
mod parsing_types;
pub use into_parser::*;
pub use parser::*;
pub use parsing_types::*;

pub use oath_parser_proc_macros::{Parse, Peek};

use oath_context::*;
use oath_src::*;
use oath_tokenizer::*;

pub trait Parse: Sized {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()>;
}

pub trait Peek: Parse {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool;
}

impl<T: Peek> Parse for Option<T> {
    #[inline(always)]
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if T::peek(parser, context) {
            T::parse(parser, context).map(|ok| Some(ok))
        } else {
            Ok(None)
        }
    }
}

impl Parse for () {
    fn parse(
        _parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        _context: ContextHandle,
    ) -> Result<Self, ()> {
        Ok(())
    }
}
