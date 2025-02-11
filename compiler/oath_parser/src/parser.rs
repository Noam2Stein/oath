use std::iter::Peekable;

use crate::*;

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

    pub fn parse<P: Parse>(&mut self, context: ContextHandle) -> Result<P, ()> {
        P::parse(self, context)
    }
    pub fn peek<P: Peek>(&mut self, context: ContextHandle) -> bool {
        P::peek(self, context)
    }

    pub fn expect_empty(&mut self, context: ContextHandle) {
        if let Some(next) = self.next() {
            let mut span = next.span();

            while let Some(next) = self.next() {
                span = span.connect(next.span());
            }

            context.push_error(Error::new("unexpected tokens", span));
        }
    }
    pub fn parse_all<P: Parse>(&mut self, context: ContextHandle) -> Result<P, ()> {
        let output = self.parse(context);
        self.expect_empty(context);

        output
    }

    pub fn end_span(&self) -> Span {
        self.end_span
    }
    pub fn next_span(&mut self) -> Span {
        if let Some(next) = self.peek_next() {
            next.span()
        } else {
            self.end_span
        }
    }

    pub fn parse_option<T: Peek>(&mut self, context: ContextHandle) -> Option<Result<T, ()>> {
        if self.peek::<T>(context) {
            Some(self.parse(context))
        } else {
            None
        }
    }

    pub fn parse_vec<T: Peek, const DISALLOW_EMPTY: bool>(
        &mut self,
        context: ContextHandle,
    ) -> Result<Vec<T>, ()> {
        let mut vec = if DISALLOW_EMPTY {
            vec![match self.parse(context) {
                Ok(first) => first,
                Err(()) => return Err(()),
            }]
        } else {
            Vec::new()
        };

        while self.peek::<T>(context) {
            if let Ok(value) = self.parse(context) {
                vec.push(value);
            }
        }

        Ok(vec)
    }

    pub fn parse_vec_all<T: Parse, const DISALLOW_EMPTY: bool>(
        &mut self,
        context: ContextHandle,
    ) -> Result<Vec<T>, ()> {
        let mut vec = if DISALLOW_EMPTY {
            vec![match self.parse(context) {
                Ok(first) => first,
                Err(()) => return Err(()),
            }]
        } else {
            Vec::new()
        };

        while self.is_left() {
            if let Ok(value) = self.parse(context) {
                vec.push(value);
            }
        }

        Ok(vec)
    }

    pub fn parse_sep<T: Peek, S: Peek, const DISALLOW_EMPTY: bool, const ALLOW_TRAIL: bool>(
        &mut self,
        context: ContextHandle,
    ) -> Result<Vec<T>, ()> {
        let mut vec = if DISALLOW_EMPTY {
            vec![match self.parse(context) {
                Ok(first) => first,
                Err(()) => return Err(()),
            }]
        } else if self.peek::<T>(context) {
            vec![match self.parse(context) {
                Ok(first) => first,
                Err(()) => return Ok(Vec::new()),
            }]
        } else {
            return Ok(Vec::new());
        };

        while let Ok(Some(_)) = self.parse::<Option<S>>(context) {
            if ALLOW_TRAIL {
                if let Ok(value) = self.parse(context) {
                    match value {
                        Some(value) => vec.push(value),
                        None => break,
                    }
                }
            } else if let Ok(value) = self.parse(context) {
                vec.push(value);
            }
        }

        Ok(vec)
    }

    pub fn parse_sep_all<
        T: Parse,
        S: Parse,
        const DISALLOW_EMPTY: bool,
        const ALLOW_TRAIL: bool,
    >(
        &mut self,
        context: ContextHandle,
    ) -> Result<Vec<T>, ()> {
        let mut vec = if DISALLOW_EMPTY {
            vec![match self.parse(context) {
                Ok(first) => first,
                Err(()) => return Err(()),
            }]
        } else if self.is_left() {
            vec![match self.parse(context) {
                Ok(first) => first,
                Err(()) => return Ok(Vec::new()),
            }]
        } else {
            return Ok(Vec::new());
        };

        while self.is_left() {
            if self.parse::<S>(context).is_err() {
                break;
            }

            if self.is_left() || !ALLOW_TRAIL {
                if let Ok(value) = self.parse(context) {
                    vec.push(value);
                }
            }
        }

        Ok(vec)
    }
}
