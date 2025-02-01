use std::sync::Mutex;

use oath_ast::ParseAstExt;
use oath_diagnostics::{Diagnostics, DiagnosticsHandle};
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
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(
            KEYWORDS
                .into_iter()
                .map(|keyword| CompletionItem::new_simple(keyword.str.to_string(), String::new()))
                .collect(),
        )))
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("You're hovering!".to_string())),
            range: None,
        }))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let diagnostics = Mutex::new(Diagnostics::default());
        let diagnostics_handle = DiagnosticsHandle(&diagnostics);

        let _ = SrcFile::from_str(&params.text_document.text)
            .tokenize(diagnostics_handle)
            .parse_ast(diagnostics_handle);

        let diagnostics = diagnostics.into_inner().unwrap();

        self.client
            .publish_diagnostics(
                params.text_document.uri,
                diagnostics
                    .errors
                    .into_iter()
                    .map(|error| {
                        Diagnostic::new_simple(span_to_range(error.span()), error.inner.to_string())
                    })
                    .collect(),
                Some(params.text_document.version),
            )
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "did change")
            .await;

        let uri = params.text_document.uri;

        let src_file = SrcFile::from_str(params.content_changes[0].text.as_str());
        let diagnostics = Mutex::new(Diagnostics::default());

        self.client
            .log_message(MessageType::INFO, "about to parse")
            .await;

        let _ = src_file
            .tokenize(DiagnosticsHandle(&diagnostics))
            .parse_ast(DiagnosticsHandle(&diagnostics));

        self.client
            .log_message(MessageType::INFO, "parsed!!!!!!")
            .await;

        if diagnostics.lock().unwrap().errors.is_empty() {
            self.client
                .log_message(MessageType::INFO, "No errors found.")
                .await;
        } else {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("Found {} errors", diagnostics.lock().unwrap().errors.len()),
                )
                .await;
        }

        let diagnostics: Vec<Diagnostic> = diagnostics
            .into_inner()
            .unwrap()
            .errors
            .into_iter()
            .map(|error| Diagnostic {
                range: Range {
                    start: Position {
                        line: error.span().start().line,
                        character: error.span().start().char,
                    },
                    end: Position {
                        line: error.span().end().line,
                        character: error.span().end().char,
                    },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: error.inner.to_string(),
                ..Default::default()
            })
            .collect();

        self.client
            .publish_diagnostics(uri, diagnostics, None)
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
