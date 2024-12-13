use std::str::FromStr;
use std::sync::{Arc, Mutex};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    documents: Arc<Mutex<std::collections::HashMap<String, String>>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // Get the document URI and content changes
        let uri = params.text_document.uri.to_string();
        let changes = params.content_changes;

        // Here we simply replace the content of the document with the new content
        // In a real server, you would use this to parse the file and find diagnostics
        let new_content = changes.last().unwrap().text.clone();

        // Generate diagnostics (e.g., simple error generation for demonstration)
        let diagnostics = self.generate_diagnostics(&new_content).await;

        {
            // Store the document content
            let mut documents = self.documents.lock().unwrap();
            documents.insert(uri.clone(), new_content);
        }

        // Send diagnostics to the client
        self.client
            .publish_diagnostics(Url::from_str(&uri).unwrap(), diagnostics, None)
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl Backend {
    // A simple diagnostic generator for demonstration
    async fn generate_diagnostics(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Example of detecting a simple error: if the file contains "error", report it
        if content.contains("error") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 5,
                    },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                source: Some("oath-language-server".to_string()),
                message: "Found an error keyword".to_string(),
                ..Default::default()
            });
        }

        diagnostics
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: Default::default(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
