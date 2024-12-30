import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';

export function activate(context: vscode.ExtensionContext) {
  // Path to the Rust server binary
  const serverCommand = context.asAbsolutePath('../target/release/editor-server');

  const serverOptions: ServerOptions = {
    run: { command: serverCommand },
    debug: { command: serverCommand }
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'oath' }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('**/.clientrc')
    }
  };

  const client = new LanguageClient('oathLanguageServer', 'Oath Language Server', serverOptions, clientOptions);

  client.start();
}