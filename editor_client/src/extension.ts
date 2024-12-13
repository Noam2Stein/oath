import * as path from "path";
import { workspace, ExtensionContext } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  // Get the path to the compiled Rust binary
  const serverModule = context.asAbsolutePath(
    path.join("target", "oath_editor_server", "debug", "oath_editor_server")  // Adjust to your target/release folder
  );

  // If the extension is launched in debug mode, use the debug server options
  const serverOptions: ServerOptions = {
    run: { module: serverModule, transport: TransportKind.ipc },
    debug: {
      module: serverModule,
      transport: TransportKind.ipc,
    },
  };

  // Options to control the language client
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "oath" }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/*.oath"),
    },
  };

  // Create the language client and start the client
  client = new LanguageClient(
    "oath",
    "Oath Language Server",
    serverOptions,
    clientOptions
  );

  // Start the client. This will also launch the server
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}