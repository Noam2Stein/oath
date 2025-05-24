use std::{
    fmt::{self, Formatter},
    sync::Arc,
};

use oath_diagnostics::*;
use oath_highlighting::*;
use oath_interner::*;
use oath_src::*;

#[derive(Debug)]
pub struct ParseContext {
    pub interner: Arc<Interner>,
    pub diagnostics: Vec<Diagnostic>,
    pub highlighter: Highlighter,
}

impl ParseContext {
    pub fn push_diagnostic(&mut self, diagnostic: impl Into<Diagnostic>) {
        self.diagnostics.push(diagnostic.into());
    }
    pub fn push_error(&mut self, diagnostic: impl Into<Error>) {
        self.diagnostics.push(Diagnostic::Error(diagnostic.into()));
    }
    pub fn push_warning(&mut self, diagnostic: impl Into<Warning>) {
        self.diagnostics.push(Diagnostic::Warning(diagnostic.into()));
    }

    pub fn intern(&mut self, str: &str) -> StrId {
        self.interner.intern(str)
    }
    pub fn unintern(&mut self, str_id: StrId) -> String {
        self.interner.unintern(str_id)
    }
    pub fn unintern_fmt(&mut self, str_id: StrId, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.interner.unintern_fmt(str_id, f)
    }

    pub fn highlight(&mut self, span: Span, color: HighlightColor) {
        self.highlighter.highlight(span, color);
    }
}
