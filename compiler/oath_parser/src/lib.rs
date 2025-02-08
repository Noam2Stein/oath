use oath_diagnostics::DiagnosticsHandle;
use oath_tokenizer::TokenTree;

mod parse_box;
mod parse_tokens;
mod parse_vec;

mod into_parser;
mod parse_garbage;
mod parser;
mod parsing_types;
pub use into_parser::*;
pub use parse_garbage::*;
pub use parser::*;
pub use parsing_types::*;

pub use oath_parser_proc_macros::{Parse, Peek};

pub trait Parse {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self;
}

pub trait Peek: Parse {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>) -> bool;
}

pub trait PeekRef: Peek {
    fn peek_ref(parser: &mut Parser<impl Iterator<Item = TokenTree>>) -> Option<&Self>;
}

pub trait TryParse: Sized {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Result<Self, ()>;
}

impl<T: Peek> Parse for Option<T> {
    #[inline(always)]
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        if T::peek(parser) {
            Some(T::parse(parser, diagnostics))
        } else {
            None
        }
    }
}

impl<T: Parse> TryParse for T {
    fn try_parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Result<Self, ()> {
        Ok(parser.parse(diagnostics))
    }
}

impl Parse for () {
    fn parse(
        _parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        _diagnostics: DiagnosticsHandle,
    ) -> Self {
    }
}
