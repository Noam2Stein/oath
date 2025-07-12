use std::collections::HashMap;
use std::sync::RwLock;
use std::{path::PathBuf, sync::Arc, thread::spawn};

use dashmap::DashMap;
use derive_more::{Deref, DerefMut};
use tower_lsp::lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, TextDocumentContentChangeEvent, Url,
};
use tower_lsp::{
    jsonrpc::Result as LspResult,
    lsp_types::{
        CompletionItem, CompletionOptions, CompletionParams, CompletionResponse, DocumentFormattingParams, Hover, HoverParams,
        HoverProviderCapability, InitializeParams, InitializeResult, OneOf, Position, Range, SemanticTokens,
        SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions, SemanticTokensParams, SemanticTokensResult,
        SemanticTokensServerCapabilities, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit,
    },
    Client, LanguageServer, LspService, Server,
};

use super::*;

pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| {
        BackendArc(Arc::new(Backend {
            client,
            root_dir: RwLock::new(PathBuf::new()),
            open_files: DashMap::new(),
            oathc: OathCompiler::new(),
            libs: RwLock::new(HashMap::new()),
        }))
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub root_dir: RwLock<PathBuf>,
    pub open_files: DashMap<Url, String>,
    pub oathc: OathCompiler,
    pub libs: RwLock<HashMap<PathBuf, LibId>>,
}

#[derive(Deref, DerefMut)]
pub struct BackendArc(pub Arc<Backend>);

#[tower_lsp::async_trait]
impl LanguageServer for BackendArc {
    async fn initialize(&self, params: InitializeParams) -> LspResult<InitializeResult> {
        let root_dir = params.root_uri.unwrap().to_file_path().unwrap();
        *self.root_dir.write().unwrap() = root_dir;

        spawn(self.watch_fn());

        Ok(InitializeResult {
            capabilities: capabilities(),
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
            data: convert_highlights(self.oathc.file_highligts(params.text_document.uri.to_file_path().unwrap())),
            result_id: None,
        })))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>, tower_lsp::jsonrpc::Error> {
        let uri = &params.text_document.uri;

        let text = self.open_files.get(uri).unwrap();

        let formatted = self.oathc.format(text.as_str());

        let range = Range {
            start: Position::new(0, 0),
            end: Position::new(u32::MAX, 0),
        };

        Ok(Some(vec![TextEdit {
            range,
            new_text: formatted,
        }]))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;

        self.open_files.insert(uri, content);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = &params.text_document.uri;
        let changes = &params.content_changes;

        apply_content_changes(&mut self.open_files.get_mut(uri).unwrap(), changes);
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;

        self.open_files.remove(&uri);
    }
}

fn capabilities() -> ServerCapabilities {
    ServerCapabilities {
        document_formatting_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        completion_provider: Some(CompletionOptions::default()),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                legend: SemanticTokensLegend {
                    token_types: HIGHLIGHT_LEGEND.into(),
                    token_modifiers: vec![],
                },
                full: Some(SemanticTokensFullOptions::Bool(true)),
                range: None,
                work_done_progress_options: Default::default(),
            },
        )),
        ..Default::default()
    }
}

fn apply_content_changes(text: &mut String, changes: &[TextDocumentContentChangeEvent]) {
    for change in changes {
        if let Some(range) = change.range {
            apply_incremental_change(text, &range, &change.text);
        } else {
            *text = change.text.clone();
        }
    }
}

fn apply_incremental_change(text: &mut String, range: &Range, new_text: &str) {
    let lines: Vec<&str> = text.lines().collect();

    let start = range.start;
    let end = range.end;

    // Get the lines and character offsets
    let mut start_byte = 0;
    let mut end_byte = 0;

    for (i, line) in lines.iter().enumerate() {
        if i < start.line as usize {
            start_byte += line.len() + 1; // +1 for '\n'
        } else if i == start.line as usize {
            start_byte += start.character as usize;
            break;
        }
    }

    for (i, line) in lines.iter().enumerate() {
        if i < end.line as usize {
            end_byte += line.len() + 1;
        } else if i == end.line as usize {
            end_byte += end.character as usize;
            break;
        }
    }

    text.replace_range(start_byte..end_byte, new_text);
}
