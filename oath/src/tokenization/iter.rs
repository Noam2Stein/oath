use super::*;

pub trait TokenIterator: Sized {
    fn peek(&mut self, errors: &mut ErrorsHandle, bound_to_line: bool) -> Option<&TokenTree>;
    fn next(&mut self, errors: &mut ErrorsHandle, bound_to_line: bool) -> Option<TokenTree>;
    fn next_line(&mut self, errors: &mut ErrorsHandle);
    fn end_span(&self) -> Span;

    fn peek_span(&mut self, errors: &mut ErrorsHandle, bound_to_line: bool) -> Span {
        if let Some(peek) = self.peek(errors, bound_to_line) {
            peek.span()
        } else {
            self.end_span()
        }
    }
    fn next_span(&mut self, errors: &mut ErrorsHandle, _bound_to_line: bool) -> Span {
        if let Some(next) = self.next(errors, true) {
            next.span()
        } else {
            self.end_span()
        }
    }

    fn parse_syntax(mut self, errors: &mut ErrorsHandle) -> crate::syntax::ModContent {
        crate::parsing::Parse::parse(&mut self, errors, false)
    }
}
pub trait IntoTokenIterator {
    fn into_token_iter(self, end_span: Span) -> impl TokenIterator;
}

impl<'src> TokenIterator for Tokenizer<'src> {
    fn peek(&mut self, errors: &mut ErrorsHandle, bound_to_line: bool) -> Option<&TokenTree> {
        self.peek(errors, bound_to_line)
    }
    fn next(&mut self, errors: &mut ErrorsHandle, bound_to_line: bool) -> Option<TokenTree> {
        self.next(errors, bound_to_line)
    }
    fn next_line(&mut self, errors: &mut ErrorsHandle) {
        self.next_line(errors);
    }
    fn end_span(&self) -> Span {
        self.src()
            .span_from_range(self.src().span().end()..self.src().span().end())
    }
}
