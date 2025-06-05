use nonempty::NonEmpty;

use super::*;

pub use oathc_span_proc_macros::OptionSpanned;

pub trait OptionSpanned {
    fn option_span(&self) -> Option<Span>;
}

impl<T: OptionSpanned> OptionSpanned for Option<T> {
    fn option_span(&self) -> Option<Span> {
        self.as_ref().map_or(None, T::option_span)
    }
}

impl<T: OptionSpanned> OptionSpanned for &T {
    fn option_span(&self) -> Option<Span> {
        T::option_span(&self)
    }
}

impl<T: OptionSpanned> OptionSpanned for Vec<T> {
    fn option_span(&self) -> Option<Span> {
        self.iter().fold(None, |a, b| a.connect(b.option_span()))
    }
}
impl<T: OptionSpanned> OptionSpanned for NonEmpty<T> {
    fn option_span(&self) -> Option<Span> {
        self.iter().fold(None, |a, b| a.connect(b.option_span()))
    }
}
