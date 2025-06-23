use super::*;

pub use oathc_fmt_proc_macros::Format;

pub trait Format {
    fn format(&self, interner: &Interner) -> FormatTree;
}

impl<T: Format> Format for Box<T> {
    fn format(&self, interner: &Interner) -> FormatTree {
        T::format(&self, interner)
    }
}

impl<T: Format> Format for Option<T> {
    fn format(&self, interner: &Interner) -> FormatTree {
        match self {
            Some(t) => t.format(interner),
            None => FormatTree::None,
        }
    }
}

impl<T: Format> Format for Try<T> {
    fn format(&self, interner: &Interner) -> FormatTree {
        match self {
            Try::Success(t) => t.format(interner),
            Try::Failure(_) => FormatTree::TryFailure,
        }
    }
}
