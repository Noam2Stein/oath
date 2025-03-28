use std::mem::{replace, MaybeUninit};

use nonempty::NonEmpty;

use crate::{
    parse_traits::{Detect, Parse},
    *,
};

#[derive(Debug, Clone)]
pub struct Parser<'ctx, I: ParserIterator> {
    iter: I,
    context: ContextHandle<'ctx>,
    last_span: Span,
}

pub trait ParserIterator {
    fn next(&mut self) -> Option<TokenTree>;

    fn peek(&self) -> Option<&TokenTree>;
}

pub struct ParserUntil<'ctx, 'p, I: ParserIterator> {
    parser: &'p mut Parser<'ctx, I>,
    f: fn(&Parser<'ctx, I>) -> bool,
}

impl<'ctx, I: ParserIterator> Parser<'ctx, I> {
    pub fn new(iter: I, context: ContextHandle<'ctx>, start: Position) -> Self {
        Self {
            iter,
            context,
            last_span: Span::from_start_len(start, 1),
        }
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        if let Some(next) = self.iter.next() {
            self.last_span = next.span();
            Some(next)
        } else {
            None
        }
    }
    pub fn peek(&self) -> Option<&TokenTree> {
        self.iter.peek()
    }
    pub fn context(&self) -> ContextHandle<'ctx> {
        self.context
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

    pub fn skip_until(&mut self, peek: impl Fn(&Self) -> bool) {
        while self.peek().is_some() && !peek(self) {
            self.next();
        }
    }

    pub fn until<'p>(&'p mut self, f: fn(&Self) -> bool) -> Parser<'ctx, ParserUntil<'ctx, 'p, I>> {
        Parser {
            context: self.context,
            last_span: self.last_span,
            iter: ParserUntil { parser: self, f },
        }
    }

    pub fn parse_rep<T: Detect>(&mut self) -> Vec<T>
    where
        Option<T>: Parse,
    {
        let mut vec = Vec::new();

        while let Some(value) = Parse::parse(self) {
            vec.push(value);
        }

        vec
    }

    pub fn try_parse_sep<T: OptionParse, S: OptionParse>(&mut self) -> Try<NonEmpty<T>> {
        let first = match T::try_parse(self) {
            Try::Success(first) => first,
            Try::Failure => return Try::Failure,
        };

        let mut vec = NonEmpty::new(first);

        while S::option_parse(self).is_some() {
            match T::try_parse(self) {
                Try::Success(value) => vec.push(value),
                Try::Failure => break,
            }
        }

        Try::Success(vec)
    }
    pub fn option_parse_sep<T: OptionParse, S: OptionParse>(&mut self) -> Option<NonEmpty<T>> {
        let first = T::option_parse(self)?;

        let mut vec = NonEmpty::new(first);

        while S::option_parse(self).is_some() {
            match T::try_parse(self) {
                Try::Success(value) => vec.push(value),
                Try::Failure => break,
            }
        }

        Some(vec)
    }

    pub fn parse_trl<T: OptionParse, S: OptionParse>(&mut self) -> Vec<T> {
        let mut vec = Vec::new();

        while let Some(value) = Option::<T>::parse(self) {
            vec.push(value);

            if let None = Option::<S>::parse(self) {
                break;
            }
        }

        vec
    }
}

impl<'ctx, I: ParserIterator> Drop for Parser<'ctx, I> {
    #[allow(dropping_copy_types)]
    #[allow(invalid_value)]
    fn drop(&mut self) {
        if let Some(next) = self.next() {
            let mut span = next.span();
            while let Some(next) = self.next() {
                span = span + next.span()
            }

            self.context
                .push_error(Error::new("Syntax Error: unexpected tokens", span));
        }

        drop(replace(&mut self.iter, unsafe {
            MaybeUninit::uninit().assume_init()
        }));
        drop(replace(&mut self.context, unsafe {
            MaybeUninit::uninit().assume_init()
        }));
        drop(replace(&mut self.last_span, unsafe {
            MaybeUninit::uninit().assume_init()
        }));
    }
}

impl<'ctx, 'p, I: ParserIterator> ParserIterator for ParserUntil<'ctx, 'p, I> {
    fn next(&mut self) -> Option<TokenTree> {
        if (self.f)(&self.parser) {
            None
        } else {
            self.parser.next()
        }
    }

    fn peek(&self) -> Option<&TokenTree> {
        if (self.f)(&self.parser) {
            None
        } else {
            self.parser.peek()
        }
    }
}

impl ParserIterator for Vec<TokenTree> {
    fn next(&mut self) -> Option<TokenTree> {
        self.pop()
    }

    fn peek(&self) -> Option<&TokenTree> {
        self.last()
    }
}
