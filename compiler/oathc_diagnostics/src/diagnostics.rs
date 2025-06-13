use std::sync::{Arc, Weak};

use dashmap::{DashMap, DashSet};

use super::*;

#[derive(Debug)]
pub struct Diagnostics(Arc<InnerDiagnostics>);

#[must_use]
#[derive(Debug, Spanned)]
pub struct DiagnosticHandle {
    index: usize,
    weak: Weak<InnerDiagnostics>,
    #[span]
    span: Span,
}

// Private

#[derive(Debug)]
struct InnerDiagnostics {
    files: DashMap<StrId, FileDiagnostics>,
    dirty_files: DashSet<StrId>,
}

type FileDiagnostics = Vec<Option<Diagnostic>>;

impl Diagnostics {
    pub fn new() -> Self {
        Self(Arc::new(InnerDiagnostics {
            files: DashMap::with_capacity(100),
            dirty_files: DashSet::new(),
        }))
    }

    pub fn arc_clone(&self) -> Self {
        Self(self.0.clone())
    }

    pub fn push_diagnostic(&self, diagnostic: impl Into<Diagnostic>) -> DiagnosticHandle {
        let diagnostic = diagnostic.into();

        let span = diagnostic.span();

        self.0.dirty_files.insert(span.file());

        let index = {
            let mut file_diagnostics_handle = self.0.files.entry(span.file()).or_default();
            let file_diagnostics = file_diagnostics_handle.value_mut();

            if let Some((index, slot)) = file_diagnostics.iter_mut().enumerate().find(|(_, slot)| slot.is_none()) {
                *slot = Some(diagnostic);

                index
            } else {
                let index = file_diagnostics.len();

                file_diagnostics.push(Some(diagnostic));

                index
            }
        };

        let weak = Arc::downgrade(&self.0);

        DiagnosticHandle { index, weak, span }
    }
    pub fn push_error(&self, diagnostic: impl Into<Error>) -> DiagnosticHandle {
        self.push_diagnostic(diagnostic.into())
    }
    pub fn push_warning(&self, diagnostic: impl Into<Warning>) -> DiagnosticHandle {
        self.push_diagnostic(diagnostic.into())
    }

    pub fn diagnostics(&self) -> impl Iterator<Item = (StrId, impl Iterator<Item = Diagnostic>)> {
        self.0.files.iter().map(|pair| {
            (
                pair.key().clone(),
                self.file_diagnostics(*pair.key()).collect::<Vec<_>>().into_iter(),
            )
        })
    }
    pub fn file_diagnostics(&self, file: StrId) -> impl Iterator<Item = Diagnostic> {
        self.0.dirty_files.remove(&file);

        self.0
            .files
            .get(&file)
            .map(|file| file.iter().cloned().collect::<Vec<_>>())
            .into_iter()
            .flatten()
            .flatten()
    }

    pub fn dirty_files(&self) -> impl Iterator<Item = (StrId, impl Iterator<Item = Diagnostic>)> {
        let mut files = Vec::with_capacity(self.0.dirty_files.len());

        self.0.dirty_files.retain(|file| {
            files.push(file.clone());

            false
        });

        files.into_iter().map(|file| {
            let diagnostics = self.file_diagnostics(file).collect::<Vec<_>>().into_iter();
            (file, diagnostics)
        })
    }
}

impl Drop for DiagnosticHandle {
    fn drop(&mut self) {
        if let Some(ctx) = self.weak.upgrade() {
            ctx.dirty_files.insert(self.span.file());

            let mut file_diagnostics_handle = ctx.files.get_mut(&self.span.file()).unwrap();
            let file_diagnostics = file_diagnostics_handle.value_mut();

            file_diagnostics[self.index] = None;
        }
    }
}
