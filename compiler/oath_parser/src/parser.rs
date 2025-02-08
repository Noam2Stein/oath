use std::iter::Peekable;

use oath_diagnostics::{DiagnosticsHandle, Error};
use oath_src::{Span, Spanned};
use oath_tokenizer::TokenTree;

use crate::{Parse, Peek, PeekRef, TryParse};

#[derive(Debug, Clone)]
pub struct Parser<I: Iterator<Item = TokenTree>> {
    iter: Peekable<I>,
    end_span: Span,
}

impl<I: Iterator<Item = TokenTree>> Iterator for Parser<I> {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<I: Iterator<Item = TokenTree>> Parser<I> {
    pub fn new(iter: Peekable<I>, end_span: Span) -> Self {
        Self { iter, end_span }
    }

    pub fn peek_next(&mut self) -> Option<&TokenTree> {
        self.iter.peek()
    }
    pub fn is_empty(&mut self) -> bool {
        self.peek_next().is_none()
    }
    pub fn is_left(&mut self) -> bool {
        self.peek_next().is_some()
    }

    pub fn parse<P: Parse>(&mut self, diagnostics: DiagnosticsHandle) -> P {
        P::parse(self, diagnostics)
    }
    pub fn peek<P: Peek>(&mut self) -> bool {
        P::peek(self)
    }
    pub fn peek_ref<P: PeekRef>(&mut self) -> Option<&P> {
        P::peek_ref(self)
    }
    pub fn try_parse<P: TryParse>(&mut self, diagnostics: DiagnosticsHandle) -> Result<P, ()> {
        P::try_parse(self, diagnostics)
    }

    pub fn expect_empty(&mut self, diagnostics: DiagnosticsHandle) {
        if let Some(next) = self.next() {
            let mut span = next.span();

            while let Some(next) = self.next() {
                span = span.connect(next.span());
            }

            diagnostics.push_error(Error::StaticMessage("unexpected tokens"), span);
        }
    }
    pub fn parse_all<P: Parse>(&mut self, diagnostics: DiagnosticsHandle) -> P {
        let output = self.parse(diagnostics);
        self.expect_empty(diagnostics);

        output
    }

    pub fn end_span(&self) -> Span {
        self.end_span
    }
}
