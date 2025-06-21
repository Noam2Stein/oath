use std::{
    path::{Path, PathBuf},
    sync::{Arc, Weak},
    thread::sleep,
};

use tower_lsp::lsp_types::{request::SemanticTokensRefresh, Diagnostic as LspDiagnostic, DiagnosticSeverity, Url};
use walkdir::WalkDir;

use super::*;

impl BackendArc {
    pub fn watch_fn(&self) -> impl FnOnce() + Send + Sync + 'static {
        let weak = Arc::downgrade(&self.0);

        move || pollster::block_on(watch(weak))
    }
}

async fn watch(backend: Weak<Backend>) {
    while let Some(backend) = backend.upgrade() {
        check_libs(&backend).await;

        for (path, diagnostics) in backend.oathc.dirty_diagnostics() {
            let diagnostics = diagnostics
                .map(|diagnostic| LspDiagnostic {
                    range: convert_span(diagnostic.span()),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: backend.oathc.format_diagnostic(&diagnostic),
                    ..Default::default()
                })
                .collect();

            backend
                .client
                .publish_diagnostics(Url::from_file_path(path).unwrap(), diagnostics, None)
                .await;
        }

        sleep(std::time::Duration::from_millis(100));
        backend.client.send_request::<SemanticTokensRefresh>(()).await.unwrap();
    }
}

async fn check_libs(backend: &Backend) {
    let wanted_dirs = find_lib_dirs(backend.root_dir.read().unwrap().as_path());
    let mut mut_dirs = backend.libs.write().unwrap();

    for added_dir in wanted_dirs
        .iter()
        .filter(|dir| !mut_dirs.contains_key(*dir))
        .collect::<Vec<_>>()
    {
        mut_dirs.insert(
            added_dir.to_path_buf(),
            backend.oathc.create_lib(added_dir.to_path_buf(), added_dir.join("oath.oh")),
        );
    }

    mut_dirs.retain(|dir, _| wanted_dirs.contains(dir));

    backend.oathc.check_lib_changes();
}

fn find_lib_dirs(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() && entry.file_name() == "oath.oh")
        .filter_map(|entry| entry.path().parent().map(|p| p.to_path_buf()))
        .collect()
}
