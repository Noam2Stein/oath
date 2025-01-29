use std::iter::Peekable;

use oath_diagnostics::DiagnosticsHandle;
use oath_tokenizer::TokenTree;

mod parse_tokens;

mod in_delimeters;
pub use in_delimeters::*;

pub trait Parse {
    fn parse(
        tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self;
}

pub trait Peek: Parse {
    fn peek(tokens: &mut Peekable<impl Iterator<Item = TokenTree>>) -> bool;
}
impl<T: Peek> Parse for Option<T> {
    #[inline(always)]
    fn parse(
        tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
        diagnostics: DiagnosticsHandle,
    ) -> Self {
        if T::peek(tokens) {
            Some(T::parse(tokens, diagnostics))
        } else {
            None
        }
    }
}

pub trait PeekRef: Peek {
    fn peek_ref(tokens: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Option<&Self>;
}

#[allow(private_bounds)]
pub trait ParseExt: Seal {
    fn parse<T: Parse>(&mut self, diagnostics: DiagnosticsHandle) -> T;

    fn parse_if<T: Peek>(&mut self, diagnostics: DiagnosticsHandle) -> Option<T>;

    fn peek<T: Peek>(&mut self) -> bool;

    fn peek_ref<T: PeekRef>(&mut self) -> Option<&T>;
}
impl<I: Iterator<Item = TokenTree>> Seal for Peekable<I> {}
impl<I: Iterator<Item = TokenTree>> ParseExt for Peekable<I> {
    #[inline(always)]
    fn parse<T: Parse>(&mut self, diagnostics: DiagnosticsHandle) -> T {
        T::parse(self, diagnostics)
    }

    #[inline(always)]
    fn parse_if<T: Peek>(&mut self, diagnostics: DiagnosticsHandle) -> Option<T> {
        Option::parse(self, diagnostics)
    }

    #[inline(always)]
    fn peek<T: Peek>(&mut self) -> bool {
        T::peek(self)
    }

    #[inline(always)]
    fn peek_ref<T: PeekRef>(&mut self) -> Option<&T> {
        T::peek_ref(self)
    }
}

trait Seal {}
