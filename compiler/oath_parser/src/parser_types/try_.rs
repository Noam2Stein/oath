use super::*;

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, OptionSpanned)]
pub enum Try<T> {
    Success(#[option_spanned] T),
    Failure,
}

impl<T> Try<T> {
    pub fn is_success(&self) -> bool {
        match self {
            Self::Success(_) => true,
            Self::Failure => false,
        }
    }
    pub fn is_failure(&self) -> bool {
        match self {
            Self::Success(_) => false,
            Self::Failure => true,
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Success(succes) => succes,
            Self::Failure => panic!("unwrapped a `failure` value"),
        }
    }
    pub fn unwrap_ref(&self) -> &T {
        match self {
            Self::Success(succes) => succes,
            Self::Failure => panic!("unwrapped a `failure` value"),
        }
    }
    pub fn unwrap_mut(&mut self) -> &mut T {
        match self {
            Self::Success(succes) => succes,
            Self::Failure => panic!("unwrapped a `failure` value"),
        }
    }

    pub fn as_ref(&self) -> Try<&T> {
        match self {
            Self::Success(success) => Try::Success(success),
            Self::Failure => Try::Failure,
        }
    }
    pub fn as_mut(&mut self) -> Try<&mut T> {
        match self {
            Self::Success(success) => Try::Success(success),
            Self::Failure => Try::Failure,
        }
    }

    pub fn map<I>(self, f: impl FnOnce(T) -> I) -> Try<I> {
        match self {
            Self::Success(success) => Try::Success(f(success)),
            Self::Failure => Try::Failure,
        }
    }

    pub fn map_box(self) -> Try<Box<T>> {
        match self {
            Self::Success(success) => Try::Success(Box::new(success)),
            Self::Failure => Try::Failure,
        }
    }
}

impl<T: ParseDesc> OptionParse for Try<T> {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
        let mut option = None;
        let exit = T::option_parse(parser, &mut option);

        *output = option.map(|value| Self::Success(value));

        exit
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        T::detect(parser)
    }
}
impl<T: ParseDesc> Parse for Try<T> {
    fn parse(parser: &mut Parser<impl Tokenizer>, output: &mut Self) -> ParseExit {
        let mut option = None;
        let exit = T::option_parse(parser, &mut option);

        if let Some(option) = option {
            *output = Try::Success(option);

            exit
        } else {
            parser
                .context()
                .push_error(SyntaxError::Expected(parser.peek_span(), T::desc()));

            *output = Try::Failure;

            ParseExit::Cut
        }
    }

    fn parse_error() -> Self {
        Try::Failure
    }
}
