use super::*;

#[must_use]
#[derive(Debug)]
pub enum Try<T> {
    Success(T),
    Failure(Option<DiagnosticHandle>),
}

impl<T: OptionSpanned> OptionSpanned for Try<T> {
    fn option_span(&self) -> Option<Span> {
        match self {
            Self::Success(t) => t.option_span(),
            Self::Failure(_) => None,
        }
    }
}

/*impl<T> Try<T> {
    pub fn success(self) -> Option<T> {
        match self {
            Self::Success(value) => Some(value),
            Self::Failure => None,
        }
    }

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
*/
