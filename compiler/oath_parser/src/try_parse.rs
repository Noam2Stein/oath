use crate::*;

pub trait TryParse: ParseDesc {
    fn try_parse(parser: &mut Parser<impl ParserIterator>) -> Try<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Try<T> {
    Success(T),
    Failure,
}

impl<T: ParseDesc> ParseDesc for Try<T> {
    fn desc() -> &'static str {
        T::desc()
    }
}
impl<T: Detect> Detect for Try<T> {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        T::detect(parser)
    }
}
impl<T: TryParse> Parse for Try<T> {
    fn parse(parser: &mut Parser<impl ParserIterator>) -> Self {
        T::try_parse(parser)
    }
}

impl<T: OptionParse> TryParse for T {
    fn try_parse(parser: &mut Parser<impl ParserIterator>) -> Try<Self> {
        if let Some(output) = T::option_parse(parser) {
            Try::Success(output)
        } else {
            parser.context().push_error(Error::new(
                format!("Syntax Error: expected {}", T::desc()),
                parser.peek_span(),
            ));

            Try::Failure
        }
    }
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

impl<T: Spanned> Try<T> {
    pub fn option_span(&self) -> Option<Span> {
        match self {
            Self::Success(success) => Some(success.span()),
            Self::Failure => None,
        }
    }
}
