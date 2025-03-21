use std::iter::Peekable;

use crate::*;

#[derive(Debug, Clone)]
pub struct Parser<I: Iterator<Item = TokenTree>> {
    iter: Peekable<I>,
    end_span: Span,
}

pub struct ParserUntilIter<'p, I: Iterator<Item = TokenTree>, F: Fn(&mut Parser<I>) -> bool> {
    parser: &'p mut Parser<I>,
    f: F,
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

    pub fn until<'p, F: Fn(&mut Self) -> bool + 'p>(
        &'p mut self,
        f: F,
    ) -> Parser<impl Iterator<Item = TokenTree> + 'p> {
        Parser {
            end_span: self.end_span,
            iter: ParserUntilIter { parser: self, f }.peekable(),
        }
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

    pub fn parse<P: Parse>(&mut self, context: ContextHandle) -> P {
        P::parse(self, context)
    }
    pub fn try_parse<P: TryParse>(&mut self, context: ContextHandle) -> Result<P, ()> {
        P::try_parse(self, context)
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

            context.push_error(Error::new("Syntax Error: unexpected tokens", span));
        }
    }
    pub fn parse_all<P: Parse>(&mut self, context: ContextHandle) -> P {
        let output = self.parse(context);
        self.expect_empty(context);

        output
    }
    pub fn try_parse_all<P: TryParse>(&mut self, context: ContextHandle) -> Result<P, ()> {
        let output = self.try_parse(context)?;
        self.expect_empty(context);

        Ok(output)
    }

    pub fn next_span(&mut self) -> Span {
        if let Some(next) = self.peek_next() {
            next.span()
        } else {
            self.end_span
        }
    }

    pub fn skip_until(&mut self, peek: impl Fn(&mut Self) -> bool) {
        while self.peek_next().is_some() && !peek(self) {
            self.next();
        }
    }

    pub fn parse_rep<T: Peek>(&mut self, context: ContextHandle) -> Vec<T>
    where
        Option<T>: Parse,
    {
        let mut vec = Vec::new();

        while let Some(value) = self.parse(context) {
            vec.push(value);
        }

        vec
    }
    pub fn parse_sep<T: Peek + Parse, S: Peek>(
        &mut self,
        context: ContextHandle,
    ) -> Result<Vec<T>, ()>
    where
        Option<S>: Parse,
    {
        if !self.peek::<T>(context) {
            context.push_error(Error::new(
                format!("Syntax Error: expected {}", T::desc()),
                self.next_span(),
            ));

            return Err(());
        }

        let mut vec = vec![self.parse(context)];

        while let Some(_) = self.parse::<Option<S>>(context) {
            vec.push(self.parse(context));
        }

        Ok(vec)
    }
    pub fn parse_trl<T: Peek + Parse, S: Peek>(&mut self, context: ContextHandle) -> Vec<T>
    where
        Option<S>: Parse,
    {
        let mut vec = Vec::new();

        while self.peek::<T>(context) {
            vec.push(self.parse(context));

            if let None = self.parse::<Option<S>>(context) {
                break;
            }
        }

        vec
    }

    pub fn try_parse_rep<T: Peek + TryParse>(&mut self, context: ContextHandle) -> Vec<PResult<T>> {
        let mut vec = Vec::new();

        while self.peek::<T>(context) {
            vec.push(self.try_parse(context));
        }

        vec
    }
    pub fn try_parse_sep<T: Peek + TryParse, S: Peek>(
        &mut self,
        context: ContextHandle,
    ) -> PResult<Vec<PResult<T>>>
    where
        Option<S>: Parse,
    {
        if !self.peek::<T>(context) {
            context.push_error(Error::new(
                format!("Syntax Error: expected {}", T::desc()),
                self.next_span(),
            ));

            return Err(());
        }

        let mut vec = vec![self.try_parse(context)];

        while let Some(_) = self.parse::<Option<S>>(context) {
            vec.push(self.try_parse(context));
        }

        Ok(vec)
    }
    pub fn try_parse_trl<T: Peek + TryParse, S: Peek>(
        &mut self,
        context: ContextHandle,
    ) -> Vec<PResult<T>>
    where
        Option<S>: Parse,
    {
        let mut vec = Vec::new();

        while self.peek::<T>(context) {
            vec.push(self.try_parse(context));

            if let None = self.parse::<Option<S>>(context) {
                break;
            }
        }

        vec
    }

    pub fn parse_rep_all<T: Parse>(&mut self, context: ContextHandle) -> Vec<T> {
        let mut vec = Vec::new();

        while self.is_left() {
            vec.push(self.parse(context));
        }

        vec
    }
    pub fn parse_trl_all<T: Parse + Peek, S: TryParse + Peek>(
        &mut self,
        context: ContextHandle,
    ) -> Vec<T> {
        let mut vec = Vec::new();

        while self.is_left() {
            vec.push(self.parse(context));

            if self.is_left() {
                match self.try_parse::<S>(context) {
                    Ok(_) => {}
                    Err(()) => {
                        while self.is_left() && !self.peek::<S>(context) {
                            self.next();
                        }

                        if self.is_left() {
                            let _ = self.try_parse::<S>(context);
                        }
                    }
                }
            } else {
                break;
            }
        }

        vec
    }

    pub fn try_parse_rep_all<T: TryParse + Peek>(
        &mut self,
        context: ContextHandle,
    ) -> Vec<PResult<T>> {
        let mut vec = Vec::new();

        while self.is_left() {
            let value = self.try_parse(context);
            if let Ok(value) = value {
                vec.push(Ok(value));
            } else {
                vec.push(Err(()));
                while self.is_left() && !self.peek::<T>(context) {
                    self.next();
                }
            }
        }

        vec
    }
    pub fn try_parse_trl_all<T: TryParse + Peek, S: TryParse + Peek>(
        &mut self,
        context: ContextHandle,
    ) -> Vec<PResult<T>> {
        let mut vec = Vec::new();

        while self.is_left() {
            match self.try_parse(context) {
                Ok(value) => {
                    vec.push(Ok(value));

                    if self.is_left() {
                        match self.try_parse::<S>(context) {
                            Ok(_) => {}
                            Err(()) => {
                                while self.is_left() && !self.peek::<S>(context) {
                                    self.next();
                                }

                                if self.is_left() {
                                    let _ = self.try_parse::<S>(context);
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
                Err(()) => {
                    vec.push(Err(()));
                    while self.is_left() && !self.peek::<T>(context) && !self.peek::<S>(context) {
                        self.next();
                    }
                }
            }
        }

        vec
    }
}

impl<'p, I: Iterator<Item = TokenTree>, F: Fn(&mut Parser<I>) -> bool> Iterator
    for ParserUntilIter<'p, I, F>
{
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.f)(self.parser) {
            None
        } else {
            self.parser.next()
        }
    }
}
