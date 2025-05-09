use std::{
    fmt::{self, Formatter},
    sync::RwLock,
};

use oath_diagnostics::*;
use oath_highlighting::*;
use oath_interner::*;
use oath_src::*;

use derive_new::new;

#[derive(Debug, new)]
pub struct Context {
    #[new(value = "RwLock::new(Diagnostics::new())")]
    pub diagnostics: RwLock<Diagnostics>,
    #[new(value = "RwLock::new(Interner::new())")]
    pub interner: RwLock<Interner>,
    #[new(value = "RwLock::new(Highlighter::new())")]
    pub highlighter: RwLock<Highlighter>,
}

impl Context {
    pub fn push_error(&self, error: impl Into<Error>) {
        self.diagnostics.write().unwrap().push_error(error);
    }
    pub fn push_warning(&self, warning: impl Into<Warning>) {
        self.diagnostics.write().unwrap().push_warning(warning);
    }
    pub fn clone_errors(&self) -> Vec<Error> {
        self.diagnostics.read().unwrap().errors().into()
    }
    pub fn clone_warnings(&self) -> Vec<Warning> {
        self.diagnostics.read().unwrap().warnings().into()
    }

    pub fn intern(&self, str: &str) -> StrId {
        self.interner.write().unwrap().intern(str)
    }
    pub fn unintern(&self, str_id: StrId) -> String {
        self.interner.read().unwrap().unintern(str_id)
    }
    pub fn unintern_fmt(&self, str_id: StrId, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.interner.read().unwrap().unintern_fmt(str_id, f)
    }

    pub fn highlight(&self, item: impl OptionSpanned, color: HighlightColor) {
        self.highlighter.write().unwrap().highlight(item, color);
    }

    pub fn clone_highlights(&self) -> Vec<(Span, HighlightColor)> {
        self.highlighter.read().unwrap().highlights().into()
    }
}
