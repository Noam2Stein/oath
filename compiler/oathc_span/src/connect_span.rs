use super::*;

pub trait ConnectSpan<Rhs> {
    type Output;

    fn connect(self, rhs: Rhs) -> Self::Output;
}

impl ConnectSpan<Span> for Span {
    type Output = Span;

    fn connect(self, rhs: Span) -> Self::Output {
        Span::from_start_end(self.start().min(rhs.start()), self.end().max(rhs.end()))
    }
}

impl ConnectSpan<Option<Span>> for Span {
    type Output = Span;

    fn connect(self, rhs: Option<Span>) -> Self::Output {
        rhs.map_or(self, |rhs| self.connect(rhs))
    }
}

impl ConnectSpan<Span> for Option<Span> {
    type Output = Span;

    fn connect(self, rhs: Span) -> Self::Output {
        self.map_or(rhs, |self_| self_.connect(rhs))
    }
}

impl ConnectSpan<Option<Span>> for Option<Span> {
    type Output = Option<Span>;

    fn connect(self, rhs: Option<Span>) -> Self::Output {
        match (self, rhs) {
            (Some(self_), Some(rhs)) => Some(self_.connect(rhs)),
            (None, Some(rhs)) => Some(rhs),
            (Some(self_), None) => Some(self_),
            (None, None) => None,
        }
    }
}
