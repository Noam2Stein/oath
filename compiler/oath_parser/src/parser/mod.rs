use nonempty::NonEmpty;

use crate::*;

mod into_parser;
pub use into_parser::*;

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

    pub fn parse_rep<T: OptionParse>(&mut self, output: &mut Vec<T>) -> ParseExit {
        loop {
            let mut item = None;
            let item_exit = T::option_parse(self, &mut item);

            if let Some(item) = item {
                output.push(item);

                if item_exit == ParseExit::Cut {
                    return ParseExit::Cut;
                }
            } else {
                return item_exit;
            }
        }
    }

    pub fn option_parse_sep<T: OptionParse, S: OptionParse>(
        &mut self,
        output: &mut Option<NonEmpty<T>>,
    ) -> ParseExit {
        let mut first = None;
        let first_exit = T::option_parse(self, &mut first);

        let first = match first {
            Some(first) => first,
            None => return first_exit,
        };

        let mut vec = NonEmpty::new(first);

        loop {
            let mut sep = None;
            let sep_exit = S::option_parse(self, &mut sep);

            if sep.is_none() || sep_exit == ParseExit::Cut {
                *output = Some(vec);
                return sep_exit;
            }

            let mut item = Try::Failure;
            let item_exit = T::try_parse(self, &mut item);

            if let Try::Success(item) = item {
                vec.push(item);

                if item_exit == ParseExit::Cut {
                    return ParseExit::Cut;
                }
            } else {
                return item_exit;
            }
        }
    }
    pub fn try_parse_sep<T: OptionParse, S: OptionParse>(
        &mut self,
        output: &mut Try<NonEmpty<T>>,
    ) -> ParseExit {
        let mut option = None;
        let exit = self.option_parse_sep::<T, S>(&mut option);

        if let Some(option) = option {
            *output = Try::Success(option);

            exit
        } else {
            self.context()
                .push_error(SyntaxError::Expected(self.peek_span(), T::desc()));

            *output = Try::Failure;

            ParseExit::Cut
        }
    }

    pub fn parse_trl<T: OptionParse, S: OptionParse>(&mut self, output: &mut Vec<T>) -> ParseExit {
        loop {
            let mut item = None;
            let item_exit = T::option_parse(self, &mut item);

            if let Some(item) = item {
                output.push(item);

                if item_exit == ParseExit::Cut {
                    return ParseExit::Cut;
                }
            } else {
                return item_exit;
            }

            let mut sep = None;
            let sep_exit = S::option_parse(self, &mut sep);

            if sep.is_none() || sep_exit == ParseExit::Cut {
                return sep_exit;
            }
        }
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

            self.context.push_error(SyntaxError::UnexpectedTokens(span));
        }
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
