use std::{
    fmt::{self, Formatter},
    sync::RwLock,
};

use oath_diagnostics::*;
use oath_highlighting::*;
use oath_interner::*;
use oath_src::*;

use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct Context {
    #[new(value = "Diagnostics::new()")]
    pub diagnostics: Diagnostics,
    #[new(value = "Interner::new()")]
    pub interner: Interner,
    #[new(value = "Highlighter::new()")]
    pub highlighter: Highlighter,
}

#[derive(Debug, Clone, Copy)]
pub struct ContextHandle<'ctx>(pub &'ctx RwLock<Context>);

impl<'ctx> ContextHandle<'ctx> {
    pub fn push_error(&self, error: impl Into<Error>) {
        self.0.write().unwrap().diagnostics.push_error(error);
    }
    pub fn push_warning(&self, warning: impl Into<Warning>) {
        self.0.write().unwrap().diagnostics.push_warning(warning);
    }
    pub fn clone_errors(&self) -> Vec<Error> {
        self.0.read().unwrap().diagnostics.errors().into()
    }
    pub fn clone_warnings(&self) -> Vec<Warning> {
        self.0.read().unwrap().diagnostics.warnings().into()
    }

    pub fn intern(&self, str: &str) -> StrId {
        self.0.write().unwrap().interner.intern(str)
    }
    pub fn unintern(&self, str_id: StrId) -> String {
        self.0.read().unwrap().interner.unintern(str_id)
    }
    pub fn unintern_fmt(&self, str_id: StrId, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.0.read().unwrap().interner.unintern_fmt(str_id, f)
    }

    pub fn highlight(&self, item: impl OptionSpanned, color: HighlightColor) {
        self.0.write().unwrap().highlighter.highlight(item, color);
    }

    pub fn clone_highlights(&self) -> Vec<(Span, HighlightColor)> {
        self.0.read().unwrap().highlighter.highlights().into()
    }
}
