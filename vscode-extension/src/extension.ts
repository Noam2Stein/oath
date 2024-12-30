import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    console.log("Hello console!");

    // Show a simple message in the VSCode status bar
    vscode.window.showInformationMessage('Hello status bar!');

    // You can also log to the output channel
    const outputChannel = vscode.window.createOutputChannel('My Language Client');
    outputChannel.appendLine("Hello output channel!");
    outputChannel.show();
}

export function deactivate() {
    console.log("Deactivating the funny VSCode extension...");
}