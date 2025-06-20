use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use std::sync::RwLock;
use std::thread;
use std::{path::PathBuf, sync::Arc};

use oathc::*;
use tower_lsp::lsp_types::request::SemanticTokensRefresh;
use tower_lsp::lsp_types::{
    CompletionItem, CompletionOptions, CompletionParams, CompletionResponse, DiagnosticSeverity, Hover, HoverParams,
    HoverProviderCapability, InitializeParams, InitializeResult, SemanticToken, SemanticTokenType, SemanticTokens,
    SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions, SemanticTokensParams, SemanticTokensResult,
    SemanticTokensServerCapabilities, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Url,
};
use tower_lsp::{lsp_types::Diagnostic as LspDiagnostic, Client, LanguageServer, LspService, Server};

mod span_range;
use span_range::*;
use walkdir::WalkDir;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| {
        BackendArc(Arc::new(Backend {
            client,
            oathc: OathCompiler::new(),
            libs: RwLock::new(HashMap::new()),
            root: RwLock::new(PathBuf::new()),
        }))
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}

#[derive(Debug)]
struct Backend {
    client: Client,
    oathc: OathCompiler,
    libs: RwLock<HashMap<PathBuf, LibId>>,
    root: RwLock<PathBuf>,
}

struct BackendArc(Arc<Backend>);

impl Deref for BackendArc {
    type Target = Backend;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for BackendArc {
    async fn initialize(&self, params: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        *self.root.write().unwrap() = params.root_uri.unwrap().to_file_path().unwrap();
        self.check_libs().await;

        let weak_self = Arc::downgrade(&self.0);
        thread::spawn(move || {
            while let Some(self_) = weak_self.upgrade() {
                pollster::block_on(self_.check_libs());

                for (path, diagnostics) in self_.oathc.dirty_diagnostics() {
                    let diagnostics = diagnostics
                        .map(|diagnostic| LspDiagnostic {
                            range: span_to_range(diagnostic.span()),
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: self_.oathc.format_diagnostic(&diagnostic),
                            ..Default::default()
                        })
                        .collect();

                    pollster::block_on(
                        self_
                            .client
                            .publish_diagnostics(Url::from_file_path(path).unwrap(), diagnostics, None),
                    );
                }

                thread::sleep(std::time::Duration::from_millis(100));
                pollster::block_on(self_.client.send_request::<SemanticTokensRefresh>(())).unwrap();
            }
        });

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
                    SemanticTokensOptions {
                        legend: SemanticTokensLegend {
                            token_types: CUSTOM_LEGEND.into(),
                            token_modifiers: vec![],
                        },
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                        range: None,
                        work_done_progress_options: Default::default(),
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn hover(&self, _: HoverParams) -> tower_lsp::jsonrpc::Result<Option<Hover>> {
        Ok(None)
    }

    async fn completion(&self, _: CompletionParams) -> tower_lsp::jsonrpc::Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(
            KEYWORDS
                .into_iter()
                .map(|keyword| CompletionItem::new_simple(keyword.to_string(), String::new()))
                .collect(),
        )))
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> tower_lsp::jsonrpc::Result<Option<SemanticTokensResult>> {
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            data: highlights_to_semantic_tokens(self.oathc.file_highligts(params.text_document.uri.to_file_path().unwrap())),
            result_id: None,
        })))
    }
}

impl Backend {
    async fn check_libs(&self) {
        let wanted_dirs = find_lib_dirs(self.root.read().unwrap().as_path());
        let mut mut_dirs = self.libs.write().unwrap();

        for added_dir in wanted_dirs
            .iter()
            .filter(|dir| !mut_dirs.contains_key(*dir))
            .collect::<Vec<_>>()
        {
            mut_dirs.insert(
                added_dir.to_path_buf(),
                self.oathc.create_lib(added_dir.to_path_buf(), added_dir.join("oath.oh")),
            );
        }

        mut_dirs.retain(|dir, _| wanted_dirs.contains(dir));

        self.oathc.check_lib_changes();
    }
}

fn highlights_to_semantic_tokens(highlights: impl Iterator<Item = Highlight>) -> Vec<SemanticToken> {
    let mut highlights = highlights.collect::<Vec<_>>();
    highlights.sort_by(
        |Highlight { span, color: _ },
         Highlight {
             span: other_span,
             color: _,
         }| span.cmp(other_span),
    );

    let mut output = Vec::new();

    let mut prev_line = 0;
    let mut prev_start = 0;

    for Highlight { span, color } in highlights {
        let delta_line = span.line().unwrap_or(0) - prev_line;
        let delta_start = if delta_line == 0 {
            span.start().char - prev_start
        } else {
            span.start().char
        };

        output.push(SemanticToken {
            delta_line: delta_line as u32,
            delta_start: delta_start as u32,
            length: span.len().unwrap_or(1),
            token_type: convert_highlight_color(color),
            token_modifiers_bitset: 0,
        });

        prev_line = span.line().unwrap_or(0);
        prev_start = span.start().char;
    }

    output
}

const CUSTOM_LEGEND: &[SemanticTokenType] = &[
    SemanticTokenType::TYPE,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::ENUM_MEMBER,
];

fn convert_highlight_color(color: HighlightColor) -> u32 {
    match color {
        HighlightColor::Green => 0,
        HighlightColor::Cyan => 1,
        HighlightColor::Yellow => 2,
        HighlightColor::Blue => 3,
    }
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
