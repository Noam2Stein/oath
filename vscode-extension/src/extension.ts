import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
} from 'vscode-languageclient/node';
import * as path from 'path';

// Declare the client variable outside of activate function
let client: LanguageClient;

// This will be called when the extension is activated
export function activate(context: vscode.ExtensionContext) {
    // Define the server command (Rust executable in your case)
    const serverExePath = path.join(context.extensionPath, '../target/debug/editor-server'); // Adjust to your Rust build output
    console.log(serverExePath)

    // Define the server options
    const serverOptions: ServerOptions = {
        run: {
            command: serverExePath,

            transport: TransportKind.stdio,
        },
        debug: {
            command: serverExePath,
            transport: TransportKind.stdio,
        },
    };

    // Define client options
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'oath' }], // Customize based on your language
        synchronize: {
            configurationSection: 'oath', // Customize to sync settings if necessary
        },
    };

    // Create the language client
    client = new LanguageClient(
        'oathLanguageServer', // Server name (you can choose whatever name you want)
        'Oath Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client
    client.start();

    // Inform the user that the server has been activated
    vscode.window.showInformationMessage('Oath Language Server is now active!');
}

// This will be called when the extension is deactivated
export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}