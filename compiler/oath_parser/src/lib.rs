mod box_impl;
mod token_impl;

mod into_parser;
mod parser;
pub use into_parser::*;
pub use parser::*;

use oath_context::*;
use oath_src::*;
use oath_tokenizer::*;

pub use oath_parser_proc_macros::{Desc, Parse, Peek, PeekOk, TryParse};

pub type PResult<T> = Result<T, ()>;

pub trait Desc: Sized {
    fn desc() -> &'static str;
}

pub trait Parse: Desc {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self;
}

pub trait TryParse: Desc {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> PResult<Self>;
}

pub trait Peek {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool;
}

pub trait PeekOk: Peek {}

impl Desc for () {
    fn desc() -> &'static str {
        "nothing"
    }
}

impl Parse for () {
    fn parse(
        _parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        _context: ContextHandle,
    ) -> Self {
    }
}

impl TryParse for () {
    fn try_parse(
        _parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        _context: ContextHandle,
    ) -> PResult<Self> {
        Ok(())
    }
}

impl<T: Peek + TryParse> TryParse for Option<T> {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if parser.peek::<T>(context) {
            Ok(Some(parser.try_parse(context)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: PeekOk + TryParse> Parse for Option<T> {
    fn parse(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> Self {
        parser.try_parse(context).unwrap()
    }
}

impl<T: Desc> Desc for Option<T> {
    fn desc() -> &'static str {
        T::desc()
    }
}
