use std::{
    path::{Path, PathBuf},
    sync::{Arc, Weak},
};

use dashmap::{DashMap, DashSet};
use oath_interner::*;
use oath_src::*;
use oath_tokens::*;

mod diagnostic;
mod try_;
pub use diagnostic::*;
pub use try_::*;

#[derive(Debug)]
pub struct Diagnostics(Arc<InnerDiagnostics>);

#[must_use]
#[derive(Debug)]
pub struct DiagnosticHandle {
    file: PathBuf,
    index: usize,
    weak: Weak<InnerDiagnostics>,
}

#[derive(Debug)]
struct InnerDiagnostics {
    files: DashMap<PathBuf, FileDiagnostics>,
    dirty_files: DashSet<PathBuf>,
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

    pub fn push_diagnostic(&self, file: impl Into<PathBuf>, diagnostic: impl Into<Diagnostic>) -> DiagnosticHandle {
        let file = file.into();
        let diagnostic = diagnostic.into();

        self.0.dirty_files.insert(file.clone());

        let index = {
            let mut file_diagnostics_handle = self.0.files.entry(file.clone()).or_default();
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

        DiagnosticHandle { file, index, weak }
    }
    pub fn push_error(&self, file: impl Into<PathBuf>, diagnostic: impl Into<Error>) -> DiagnosticHandle {
        self.push_diagnostic(file, diagnostic.into())
    }
    pub fn push_warning(&self, file: impl Into<PathBuf>, diagnostic: impl Into<Warning>) -> DiagnosticHandle {
        self.push_diagnostic(file, diagnostic.into())
    }

    pub fn diagnostics(&self) -> impl Iterator<Item = (PathBuf, impl Iterator<Item = Diagnostic>)> {
        self.0.files.iter().map(|pair| {
            (
                pair.key().clone(),
                self.file_diagnostics(pair.key()).collect::<Vec<_>>().into_iter(),
            )
        })
    }
    pub fn file_diagnostics(&self, file: impl AsRef<Path>) -> impl Iterator<Item = Diagnostic> {
        let file = file.as_ref();

        self.0.dirty_files.remove(file);

        self.0
            .files
            .get(file)
            .map(|file| file.iter().cloned().collect::<Vec<_>>())
            .into_iter()
            .flatten()
            .flatten()
    }

    pub fn dirty_files(&self) -> impl Iterator<Item = (PathBuf, impl Iterator<Item = Diagnostic>)> {
        let mut files = Vec::with_capacity(self.0.dirty_files.len());

        self.0.dirty_files.retain(|file| {
            files.push(file.clone());

            false
        });

        files.into_iter().map(|file| {
            let diagnostics = self.file_diagnostics(&file).collect::<Vec<_>>().into_iter();
            (file, diagnostics)
        })
    }
}

impl Drop for DiagnosticHandle {
    fn drop(&mut self) {
        if let Some(ctx) = self.weak.upgrade() {
            ctx.dirty_files.insert(self.file.clone());

            let mut file_diagnostics_handle = ctx.files.get_mut(&self.file).unwrap();
            let file_diagnostics = file_diagnostics_handle.value_mut();

            file_diagnostics[self.index] = None;
        }
    }
}
