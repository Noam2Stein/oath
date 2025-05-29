use std::{
    iter::Flatten,
    path::{Path, PathBuf},
    slice::Iter,
    sync::{Arc, Weak},
};

use dashmap::{DashMap, DashSet};
use oath_interner::*;
use oath_src::*;
use oath_tokens::*;

mod diagnostic;
pub use diagnostic::*;

#[derive(Debug)]
pub struct Diagnostics(Arc<InnerDiagnostics>);

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

    pub fn clean_dirty_files(&self, mut f: impl FnMut(PathBuf, Flatten<Iter<Option<Diagnostic>>>)) {
        self.0.dirty_files.retain(|file| {
            let file_diagnostics = self.0.files.get(file).unwrap();
            let iter = file_diagnostics.iter().flatten();

            f(file.clone(), iter);

            false
        });
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
