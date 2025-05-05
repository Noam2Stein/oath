use nonempty::NonEmpty;

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

    pub fn skip_until(&mut self, peek: impl Fn(&Self) -> bool) {
        while self.peek().is_some() && !peek(self) {
            self.next();
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

    pub fn option_parse_sep<T: OptionParse, S: OptionParse>(&mut self, output: &mut Option<NonEmpty<T>>) -> ParseExit {
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
    pub fn try_parse_sep<T: OptionParse, S: OptionParse>(&mut self, output: &mut Try<NonEmpty<T>>) -> ParseExit {
        let mut option = None;
        let exit = self.option_parse_sep::<T, S>(&mut option);

        if let Some(option) = option {
            *output = Try::Success(option);

            exit
        } else {
            self.context().push_error(SyntaxError::Expected(self.peek_span(), T::desc()));

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
