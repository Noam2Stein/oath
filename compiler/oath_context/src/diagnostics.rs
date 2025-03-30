use std::mem::replace;

use oath_src::{Span, Spanned};

use crate::ContextHandle;

#[derive(Debug, Clone, Hash, Spanned)]
pub struct Error {
    #[span]
    span: Span,
    pub message: String,
}

#[derive(Debug, Clone, Hash, Spanned)]
pub struct Warning {
    #[span]
    span: Span,
    pub message: String,
}

#[derive(Debug)]
pub(super) struct Diagnostics {
    errors: Vec<Error>,
    warnings: Vec<Warning>,
}

impl<'ctx> ContextHandle<'ctx> {
    pub fn push_error(self, error: impl Into<Error>) {
        self.0.lock().unwrap().diagnostics.errors.push(error.into());
    }
    pub fn push_warning(self, warning: impl Into<Warning>) {
        self.0
            .lock()
            .unwrap()
            .diagnostics
            .warnings
            .push(warning.into());
    }

    pub fn collect_errors(self) -> Vec<Error> {
        replace(&mut self.0.lock().unwrap().diagnostics.errors, Vec::new())
    }
    pub fn collect_warnings(self) -> Vec<Warning> {
        replace(&mut self.0.lock().unwrap().diagnostics.warnings, Vec::new())
    }
}

impl Error {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

impl Warning {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}
