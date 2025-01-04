mod error;
use std::sync::Mutex;

pub use error::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Diagnostics {
    pub errors: Vec<Error>,
}

#[derive(Debug, Clone, Copy)]
pub struct DiagnosticsHandle<'d>(pub &'d Mutex<Diagnostics>);
impl<'d> DiagnosticsHandle<'d> {
    #[inline(always)]
    pub fn push_error(self, error: Error) {
        self.0.lock().unwrap().errors.push(error);
    }
}
