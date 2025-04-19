use crate::*;

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
