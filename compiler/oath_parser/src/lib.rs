mod box_impl;
mod token_impl;

mod into_parser;
mod parser;
pub use into_parser::*;
pub use parser::*;

pub use oath_parser_proc_macros::{Parse, Peek};

use oath_context::*;
use oath_src::*;
use oath_tokenizer::*;

pub trait Desc: Sized {
    fn desc() -> &'static str;
}

pub trait Parse {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self;
}

pub trait TryParse: Desc {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()>;
}

pub trait Peek: Desc {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool;
}

impl<T: TryParse> Parse for Result<T, ()> {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        parser.try_parse(context)
    }
}

impl<T: Desc> Desc for Result<T, ()> {
    fn desc() -> &'static str {
        T::desc()
    }
}
