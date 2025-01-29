use std::sync::Mutex;

mod desc;
mod error;
mod fill;
pub use desc::*;
pub use error::*;
pub use fill::*;
use oath_src::{Span, WithSpan};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Diagnostics {
    pub errors: Vec<WithSpan<Error>>,
}

#[derive(Debug, Clone, Copy)]
pub struct DiagnosticsHandle<'d>(pub &'d Mutex<Diagnostics>);
impl<'d> DiagnosticsHandle<'d> {
    #[inline(always)]
    pub fn push_error(self, error: Error, span: Span) {
        self.0
            .lock()
            .unwrap()
            .errors
            .push(WithSpan::new(error, span));
    }
}
