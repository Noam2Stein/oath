use std::{
    fmt::{self, Formatter},
    path::PathBuf,
    sync::Arc,
};

use oath_diagnostics::*;
use oath_highlighting::*;
use oath_interner::*;
use oath_src::*;

#[derive(Debug)]
pub struct ParseContext {
    pub path: PathBuf,
    pub interner: Arc<Interner>,
    pub diagnostics: Diagnostics,
    pub highlighter: Highlighter,
}

impl ParseContext {
    pub fn push_diagnostic(&mut self, diagnostic: impl Into<Diagnostic>) -> DiagnosticHandle {
        self.diagnostics.push_diagnostic(self.path.clone(), diagnostic)
    }
    pub fn push_error(&mut self, diagnostic: impl Into<Error>) -> DiagnosticHandle {
        self.diagnostics.push_error(self.path.clone(), diagnostic)
    }
    pub fn push_warning(&mut self, diagnostic: impl Into<Warning>) -> DiagnosticHandle {
        self.diagnostics.push_warning(self.path.clone(), diagnostic)
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

    pub fn highlight(&mut self, target: impl Highlight, color: HighlightColor) {
        target.highlight(color, &mut self.highlighter);
    }
    pub fn highlight_span(&mut self, span: Span, color: HighlightColor) {
        self.highlighter.highlight_span(span, color);
    }
}
