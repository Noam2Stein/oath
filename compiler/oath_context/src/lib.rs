use std::sync::Mutex;

use derive_new::new;
use oath_diagnostics::*;
use oath_highlighting::*;
use oath_interner::*;

#[derive(Debug, Clone, new)]
pub struct Context {
    #[new(value = "Diagnostics::new()")]
    diagnostics: Diagnostics,
    #[new(value = "Interner::new()")]
    interner: Interner,
    #[new(value = "Highlighter::new()")]
    highlighter: Highlighter,
}

#[derive(Debug, Clone, Copy)]
pub struct ContextHandle<'ctx>(pub &'ctx Mutex<Context>);

impl<'ctx> ContextHandle<'ctx> {
    pub fn push_error(&self, error: impl Into<Error>) {
        self.0.lock().unwrap().diagnostics.push_error(error);
    }
    pub fn push_warning(&self, warning: impl Into<Warning>) {
        self.0.lock().unwrap().diagnostics.push_warning(warning);
    }
    pub fn clone_errors(&self) -> Vec<Error> {
        self.0.lock().unwrap().diagnostics.errors().into()
    }
    pub fn clone_warnings(&self) -> Vec<Warning> {
        self.0.lock().unwrap().diagnostics.errors().into()
    }
}
