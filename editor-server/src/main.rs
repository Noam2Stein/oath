use std::sync::Mutex;

use oath_ast::ParseAstExt;
use oath_context::{Context, ContextHandle};
use oath_src::{Spanned, SrcFile};
use oath_tokenizer::{SrcFileTokenizeExt, KEYWORDS};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod span_range;
use span_range::*;

#[derive(Debug)]
struct Backend {
    client: Client,
}

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
}

impl Backend {
    async fn validate_file(&self, uri: Url, text: &str, version: i32) {
        let src_file = SrcFile::from_str(text);

        let context = Mutex::new(Context::new());
        let context = ContextHandle(&context);

        let _ = src_file.tokenize(context).parse_ast(context);

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

        self.client
            .publish_diagnostics(uri, diagnostics, Some(version))
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
