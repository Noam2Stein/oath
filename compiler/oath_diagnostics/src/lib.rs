mod error;
pub use error::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Diagnostics {
    pub errors: Vec<Error>,
}
impl Diagnostics {
    #[inline(always)]
    pub fn handle(&mut self) -> DiagnosticsHandle {
        DiagnosticsHandle { diagnostics: self }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticsHandle<'d> {
    diagnostics: &'d mut Diagnostics,
}
impl<'d> DiagnosticsHandle<'d> {
    #[inline(always)]
    pub fn push_error(&mut self, error: Error) {
        self.diagnostics.errors.push(error);
    }
}
