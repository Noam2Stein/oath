use std::collections::HashMap;
use std::sync::Mutex;

use oath_ast::ParseAstExt;
use oath_context::{Context, ContextHandle, HighlightColor};
//use oath_name_res::{DumbNameContext, IntoNamespace};
use oath_src::{Span, Spanned, SrcFile};
use oath_tokenizer::{SrcFileTokenizeExt, KEYWORDS};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod span_range;
use span_range::*;

#[derive(Debug)]
struct Backend {
    client: Client,
    highlights: Mutex<HashMap<Url, Vec<SemanticToken>>>,
}

const CUSTOM_LEGEND: &[SemanticTokenType] = &[
    SemanticTokenType::TYPE,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::STRING,
    SemanticTokenType::NUMBER,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::COMMENT,
];

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: CUSTOM_LEGEND.into(),
                                token_modifiers: vec![],
                            },
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            range: None,
                            work_done_progress_options: Default::default(),
                        },
                    ),
                ),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Oath lang server initiated")
            .await;
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

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;

        let highlights = self.highlights.lock().unwrap();
        let tokens = highlights.get(&uri).cloned().unwrap_or_default();

        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            data: tokens,
            result_id: None,
        })))
    }
}

impl Backend {
    async fn validate_file(&self, uri: Url, text: &str, version: i32) {
        let src_file = SrcFile::from_str(text);

        let context = Mutex::new(Context::new());
        let context = ContextHandle(&context);

        {
            let _ = src_file.tokenize(context).parse_ast(context);

            //let mut name_context = DumbNameContext::new();
            //let _ = ast.into_namespace(&mut name_context, context);
            //let _ = name_context.resolve();
        }

        let diagnostics: Vec<Diagnostic> = context
            .collect_errors()
            .into_iter()
            .map(|error| Diagnostic {
                range: span_to_range(error.span()),
                severity: Some(DiagnosticSeverity::ERROR),
                message: error.message.to_string(),
                ..Default::default()
            })
            .collect();

        let highlights = context.collect_highlights();

        self.highlights
            .lock()
            .unwrap()
            .insert(uri.clone(), highlights_to_semantic_tokens(&highlights));

        self.client
            .publish_diagnostics(uri, diagnostics, Some(version))
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        highlights: Default::default(),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}

fn highlights_to_semantic_tokens(highlights: &[(Span, HighlightColor)]) -> Vec<SemanticToken> {
    let mut output = Vec::new();

    let mut prev_line = 0;
    let mut prev_start = 0;

    for (span, color) in highlights {
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
            token_type: color_to_token_type(*color),
            token_modifiers_bitset: 0,
        });

        prev_line = span.line().unwrap_or(0);
        prev_start = span.start().char;
    }

    output
}

fn color_to_token_type(color: HighlightColor) -> u32 {
    match color {
        HighlightColor::Green => 0,
        HighlightColor::Cyan => 1,
        HighlightColor::Yellow => 2,
    }
}
