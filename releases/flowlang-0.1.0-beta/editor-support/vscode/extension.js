// FlowLang VS Code Extension
// This extension provides syntax highlighting for FlowLang programming language

const vscode = require('vscode');

/**
 * This method is called when your extension is activated
 * Your extension is activated the very first time the command is executed
 */
function activate(context) {
    console.log('FlowLang syntax highlighting extension is now active!');

    // Register any commands or providers here if needed in the future
    // For now, the extension only provides syntax highlighting through grammar files
}

/**
 * This method is called when your extension is deactivated
 */
function deactivate() {
    console.log('FlowLang syntax highlighting extension is deactivated.');
}

module.exports = {
    activate,
    deactivate
};