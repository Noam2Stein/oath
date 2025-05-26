use std::path::PathBuf;
use std::sync::RwLock;

use oath::*;
use tower_lsp::{
    jsonrpc::Result, lsp_types::Diagnostic as LspDiagnostic, lsp_types::*, Client, LanguageServer, LspService, Server,
};

mod span_range;
use span_range::*;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        oath: OathCompiler::new(),
        lib: RwLock::new(None),
        root: RwLock::new(PathBuf::new()),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}

#[derive(Debug)]
struct Backend {
    client: Client,
    oath: OathCompiler,
    lib: RwLock<Option<LibId>>,
    root: RwLock<PathBuf>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        *self.root.write().unwrap() = params.root_uri.unwrap().to_file_path().unwrap();
        *self.lib.write().unwrap() = Some(self.oath.create_lib(self.root.read().unwrap().to_path_buf(), []));

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

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(None)
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(
            KEYWORDS
                .into_iter()
                .map(|keyword| CompletionItem::new_simple(keyword.to_string(), String::new()))
                .collect(),
        )))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.validate_file(
            params.text_document.uri,
            params.text_document.text.as_str(),
            params.text_document.version,
        )
        .await
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.validate_file(
            params.text_document.uri,
            params.content_changes[0].text.as_str(),
            params.text_document.version,
        )
        .await
    }

    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> Result<Option<SemanticTokensResult>> {
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            data: highlights_to_semantic_tokens(self.oath.get_mod_highligts(params.text_document.uri.to_file_path().unwrap())),
            result_id: None,
        })))
    }
}

impl Backend {
    async fn validate_file(&self, uri: Url, text: &str, version: i32) {
        let parser_diagnostics = self.oath.parse_text(text);

        let diagnostics = self
            .oath
            .get_mod_semantic_diagnostics(uri.to_file_path().unwrap())
            .chain(parser_diagnostics)
            .map(|diagnostic| LspDiagnostic {
                range: span_to_range(diagnostic.span()),
                severity: Some(DiagnosticSeverity::ERROR),
                message: self.oath.format_diagnostic(&diagnostic),
                ..Default::default()
            })
            .collect();

        self.client.publish_diagnostics(uri, diagnostics, Some(version)).await;
    }
}

fn highlights_to_semantic_tokens(highlights: impl Iterator<Item = HighlightInfo>) -> Vec<SemanticToken> {
    let mut highlights = highlights.collect::<Vec<_>>();
    highlights.sort_by(
        |HighlightInfo { span, color: _ },
         HighlightInfo {
             span: other_span,
             color: _,
         }| span.cmp(other_span),
    );

    let mut output = Vec::new();

    let mut prev_line = 0;
    let mut prev_start = 0;

    for HighlightInfo { span, color } in highlights {
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
