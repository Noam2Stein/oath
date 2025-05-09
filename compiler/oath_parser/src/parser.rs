use crate::*;

pub struct Parser<'src, 'ctx, 'parent> {
    tokenizer: Tokenizer<'src, 'ctx, 'parent>,
    last_span: Span,
}

impl<'src, 'ctx, 'parent> Parser<'src, 'ctx, 'parent> {
    pub fn new(tokenizer: Tokenizer<'src, 'ctx, 'parent>) -> Self {
        Self {
            last_span: tokenizer
                .open_delimeter()
                .map_or(Span::from_start_len(Position::ZERO, 1), |open| open.span),
            tokenizer,
        }
    }

    pub fn next(&mut self) -> Option<LazyToken<'src, 'ctx, '_>> {
        if let Some(next) = self.tokenizer.next() {
            self.last_span = next.span();
            Some(next)
        } else {
            None
        }
    }
    pub fn peek(&self) -> Option<PeekToken> {
        self.tokenizer.peek()
    }
    pub fn context(&self) -> ContextHandle<'ctx> {
        self.tokenizer.context()
    }

    pub fn peek_span(&self) -> Span {
        if let Some(next) = self.peek() {
            let span = next.span();

            if span.start().line == self.last_span.end().line {
                span
            } else {
                Span::from_start_len(self.last_span.end(), 1)
            }
        } else {
            Span::from_start_len(self.last_span.end(), 1)
        }
    }
    pub fn is_empty(&self) -> bool {
        self.peek().is_none()
    }
    pub fn is_not_empty(&self) -> bool {
        self.peek().is_some()
    }
}

impl<'src, 'ctx, 'parent> Drop for Parser<'src, 'ctx, 'parent> {
    fn drop(&mut self) {
        let mut span = match self.next() {
            Some(next) => next.span(),
            None => return,
        };

        while let Some(next) = self.next() {
            span = span + next.span()
        }

        self.context().push_error(SyntaxError::UnexpectedTokens(span));
    }
}
