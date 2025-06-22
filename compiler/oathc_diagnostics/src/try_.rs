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

impl<T: Highlightable> Highlightable for Try<T> {
    fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
        match self {
            Self::Success(t) => t.highlight(color, h),
            Self::Failure(_) => {}
        }
    }
}

impl<T> Try<T> {
    pub fn success_ref(&self) -> Option<&T> {
        match self {
            Self::Success(success) => Some(success),
            Self::Failure(_) => None,
        }
    }
    pub fn success_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Success(success) => Some(success),
            Self::Failure(_) => None,
        }
    }

    pub fn unwrap_ref(&self) -> &T {
        match self {
            Self::Success(succes) => succes,
            Self::Failure(_) => panic!("unwrapped a `failure` value"),
        }
    }
    pub fn unwrap_mut(&mut self) -> &mut T {
        match self {
            Self::Success(succes) => succes,
            Self::Failure(_) => panic!("unwrapped a `failure` value"),
        }
    }

    pub fn map<O>(self, f: impl FnOnce(T) -> O) -> Try<O> {
        match self {
            Self::Success(t) => Try::Success(f(t)),
            Self::Failure(handle) => Try::Failure(handle),
        }
    }
}

impl<T: InternedDisplay> InternedDisplay for Try<T> {
    fn interned_fmt(&self, f: &mut std::fmt::Formatter, interner: &Interner) -> std::fmt::Result {
        match self {
            Self::Success(t) => t.interned_fmt(f, interner),
            Self::Failure(_) => Ok(()),
        }
    }
}
