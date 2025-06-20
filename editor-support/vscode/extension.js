// FlowLang Language Server Extension
// Provides syntax highlighting, error detection, diagnostics, and code completion

const vscode = require('vscode');
const { spawn } = require('child_process');
const path = require('path');

let diagnosticCollection;
let outputChannel;

// FlowLang keywords and built-in functions for code completion
const FLOWLANG_KEYWORDS = [
    'let', 'def', 'if', 'then', 'else', 'end', 'while', 'for', 'from', 'to', 'do',
    'return', 'break', 'continue', 'with', 'be', 'and', 'or', 'not', 'show',
    'true', 'false', 'null', 'import', 'export', 'try', 'catch', 'in'
];

const FLOWLANG_FUNCTIONS = [
    { name: 'show', detail: 'show(value)', documentation: 'Display a value to the output' },
    { name: 'len', detail: 'len(collection)', documentation: 'Get the length of a collection' },
    { name: 'type', detail: 'type(value)', documentation: 'Get the type of a value' },
    { name: 'str', detail: 'str(value)', documentation: 'Convert value to string' },
    { name: 'num', detail: 'num(value)', documentation: 'Convert value to number' },
    { name: 'bool', detail: 'bool(value)', documentation: 'Convert value to boolean' },
    { name: 'print', detail: 'print(value)', documentation: 'Print a value without newline' },
    { name: 'int', detail: 'int(value)', documentation: 'Convert value to integer' },
    { name: 'float', detail: 'float(value)', documentation: 'Convert value to float' }
];

// FlowLang built-in functions
const builtinFunctions = new Set([
    'show', 'print', 'len', 'type', 'str', 'int', 'float', 'num', 'bool'
]);

// FlowLang standard library functions
const stdlibFunctions = new Set([
    // I/O functions
    'read_file', 'write_file', 'append_file', 'read_lines', 'copy_file',
    'file_exists', 'is_directory', 'is_file', 'create_dir', 'remove_path',
    'list_dir', 'file_size',
    // System functions
    'get_env', 'set_env', 'remove_env', 'get_all_env', 'execute_command',
    'get_current_dir', 'change_dir', 'get_args', 'exit_program',
    // Network functions
    'http_get', 'http_post', 'http_put', 'http_delete', 'url_encode', 'url_decode',
    // JSON functions
    'json_parse', 'json_stringify',
    // Crypto functions
    'hash_string', 'md5_hash', 'sha256_hash', 'base64_encode', 'base64_decode',
    'hex_encode', 'hex_decode', 'random_int', 'random_float', 'random_string',
    'set_random_seed'
]);

// FlowLang keywords and literals
const flowlangKeywords = new Set(FLOWLANG_KEYWORDS);

const FLOWLANG_SNIPPETS = [
    {
        label: 'if-then-else',
        insertText: new vscode.SnippetString('if ${1:condition} then\n\t${2:// code}\nelse\n\t${3:// code}\nend'),
        documentation: 'If-then-else statement'
    },
    {
        label: 'while-loop',
        insertText: new vscode.SnippetString('while ${1:condition} do\n\t${2:// code}\nend'),
        documentation: 'While loop'
    },
    {
        label: 'for-loop',
        insertText: new vscode.SnippetString('for ${1:i} from ${2:0} to ${3:10} do\n\t${4:// code}\nend'),
        documentation: 'For loop'
    },
    {
        label: 'function',
        insertText: new vscode.SnippetString('def ${1:functionName} with ${2:parameters} do\n\t${3:// code}\n\treturn ${4:result}\nend'),
        documentation: 'Function definition with parameters'
    },
    {
        label: 'function (no params)',
        insertText: new vscode.SnippetString('def ${1:functionName} do\n\t${2:// code}\n\treturn ${3:result}\nend'),
        documentation: 'Function definition without parameters'
    },
    {
        label: 'let-variable',
        insertText: new vscode.SnippetString('let ${1:variableName} be ${2:value}'),
        documentation: 'Variable declaration'
    }
];

class FlowLangCompletionProvider {
    provideCompletionItems(document, position, token, context) {
        const config = vscode.workspace.getConfiguration('flowlang');
        if (!config.get('enableCodeCompletion', true)) {
            return [];
        }

        const linePrefix = document.lineAt(position).text.substr(0, position.character);
        const completionItems = [];

        // Add keywords
        FLOWLANG_KEYWORDS.forEach(keyword => {
            const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
            item.detail = 'FlowLang keyword';
            completionItems.push(item);
        });

        // Add built-in functions
        FLOWLANG_FUNCTIONS.forEach(func => {
            const item = new vscode.CompletionItem(func.name, vscode.CompletionItemKind.Function);
            item.detail = func.detail;
            item.documentation = new vscode.MarkdownString(func.documentation);
            item.insertText = new vscode.SnippetString(`${func.name}(\${1:})`);
            completionItems.push(item);
        });
        
        // Add standard library functions
        stdlibFunctions.forEach(func => {
            const item = new vscode.CompletionItem(func, vscode.CompletionItemKind.Function);
            item.detail = 'Standard library function';
            item.documentation = new vscode.MarkdownString(`Standard library function: \`${func}\``);
            completionItems.push(item);
        });

        // Add snippets
        FLOWLANG_SNIPPETS.forEach(snippet => {
            const item = new vscode.CompletionItem(snippet.label, vscode.CompletionItemKind.Snippet);
            item.insertText = snippet.insertText;
            item.documentation = new vscode.MarkdownString(snippet.documentation);
            completionItems.push(item);
        });

        // Add variables from current document
        const text = document.getText();
        const variableMatches = text.match(/let\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+be/g);
        if (variableMatches) {
            const variables = new Set();
            variableMatches.forEach(match => {
                const varName = match.match(/let\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+be/)[1];
                if (!variables.has(varName)) {
                    variables.add(varName);
                    const item = new vscode.CompletionItem(varName, vscode.CompletionItemKind.Variable);
                    item.detail = 'Local variable';
                    completionItems.push(item);
                }
            });
        }

        // Add functions from current document
        const functionMatches = text.match(/def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+(with|do)/g);
        if (functionMatches) {
            const functions = new Set();
            functionMatches.forEach(match => {
                const funcName = match.match(/def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+(with|do)/)[1];
                if (!functions.has(funcName)) {
                    functions.add(funcName);
                    const item = new vscode.CompletionItem(funcName, vscode.CompletionItemKind.Function);
                    item.detail = 'User-defined function';
                    completionItems.push(item);
                }
            });
        }

        return completionItems;
    }
}

class FlowLangHoverProvider {
    provideHover(document, position, token) {
        const range = document.getWordRangeAtPosition(position);
        if (!range) return;

        const word = document.getText(range);
        
        // Provide hover information for keywords
        const keywordInfo = {
            'let': 'Declares a variable: `let variableName be value`',
            'def': 'Defines a function: `def functionName with parameters do ... end` or `def functionName do ... end`',
            'if': 'Conditional statement: `if condition then ... else ... end`',
            'while': 'Loop statement: `while condition do ... end`',
            'for': 'For loop: `for variable from start to end do ... end`',
            'show': 'Built-in function to display output: `show(value)`',
            'return': 'Returns a value from a function',
            'break': 'Exits from a loop',
            'continue': 'Continues to next iteration of a loop',
            'import': 'Imports a module or library',
            'export': 'Exports a function or variable',
            'try': 'Begins a try-catch block for error handling',
            'catch': 'Catches exceptions in a try-catch block'
        };
        
        // Built-in functions hover info
        const builtinFunctionDocs = {
            'show': 'Prints a value to stdout with a newline',
            'print': 'Prints a value to stdout without a newline',
            'len': 'Returns the length of a string, array, or object',
            'type': 'Returns the type of a value as a string',
            'str': 'Converts a value to a string',
            'int': 'Converts a value to an integer',
            'float': 'Converts a value to a floating-point number',
            'num': 'Converts a value to a number',
            'bool': 'Converts a value to a boolean'
        };
        
        // Standard library functions hover info
        const stdlibFunctionDocs = {
            'read_file': 'Reads the contents of a file',
            'write_file': 'Writes content to a file',
            'append_file': 'Appends content to a file',
            'file_exists': 'Checks if a file exists',
            'json_parse': 'Parses a JSON string into a FlowLang value',
            'json_stringify': 'Converts a FlowLang value to a JSON string',
            'http_get': 'Performs an HTTP GET request',
            'http_post': 'Performs an HTTP POST request',
            'random_int': 'Generates a random integer',
            'hash_string': 'Computes a hash of a string'
        };

        if (keywordInfo[word]) {
            return new vscode.Hover(new vscode.MarkdownString(`**${word}**\n\n${keywordInfo[word]}`));
        }

        // Provide hover for built-in functions
        const func = FLOWLANG_FUNCTIONS.find(f => f.name === word);
        if (func) {
            return new vscode.Hover(new vscode.MarkdownString(`**${func.detail}**\n\n${func.documentation}`));
        }
        
        if (builtinFunctionDocs[word]) {
            return new vscode.Hover(new vscode.MarkdownString(`**${word}** (built-in function)\n\n${builtinFunctionDocs[word]}`));
        }
        
        if (stdlibFunctionDocs[word]) {
            return new vscode.Hover(new vscode.MarkdownString(`**${word}** (standard library function)\n\n${stdlibFunctionDocs[word]}`));
        }

        return null;
    }
}

class FlowLangDiagnosticProvider {
    static async provideDiagnostics(document) {
        const config = vscode.workspace.getConfiguration('flowlang');
        if (!config.get('enableDiagnostics', true)) {
            return [];
        }

        const diagnostics = [];
        const text = document.getText();
        const lines = text.split('\n');

        // Basic syntax checking
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i].trim();
            const lineNumber = i;

            // Check for common syntax errors
            if (line.startsWith('let ') && !line.includes(' be ')) {
                const diagnostic = new vscode.Diagnostic(
                    new vscode.Range(lineNumber, 0, lineNumber, line.length),
                    'Variable declaration missing "be" keyword. Use: let variableName be value',
                    vscode.DiagnosticSeverity.Error
                );
                diagnostic.code = 'missing-be-keyword';
                diagnostics.push(diagnostic);
            }

            // Check for unmatched brackets
            const openBrackets = (line.match(/[\(\[\{]/g) || []).length;
            const closeBrackets = (line.match(/[\)\]\}]/g) || []).length;
            if (openBrackets !== closeBrackets) {
                const diagnostic = new vscode.Diagnostic(
                    new vscode.Range(lineNumber, 0, lineNumber, line.length),
                    'Unmatched brackets detected',
                    vscode.DiagnosticSeverity.Warning
                );
                diagnostic.code = 'unmatched-brackets';
                diagnostics.push(diagnostic);
            }

            // Check for incomplete if statements
            if (line.includes('if ') && !line.includes(' then')) {
                const diagnostic = new vscode.Diagnostic(
                    new vscode.Range(lineNumber, 0, lineNumber, line.length),
                    'If statement missing "then" keyword',
                    vscode.DiagnosticSeverity.Error
                );
                diagnostic.code = 'missing-then-keyword';
                diagnostics.push(diagnostic);
            }

            // Check for incomplete function definitions
            if (line.startsWith('def ') && !line.includes('with') && !line.includes('do')) {
                const diagnostic = new vscode.Diagnostic(
                    new vscode.Range(lineNumber, 0, lineNumber, line.length),
                    'Function definition missing "with" or "do" keyword',
                    vscode.DiagnosticSeverity.Error
                );
                diagnostic.code = 'missing-parameters';
                diagnostics.push(diagnostic);
            }

            // Check for undefined variables (basic check)
            const variableUsage = line.match(/\b([a-zA-Z_][a-zA-Z0-9_]*)\b/g);
            if (variableUsage) {
                const definedVars = new Set(['true', 'false', 'null', 'show']);
                
                // Add variables defined in the document
                const varMatches = text.match(/let\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+be/g);
                if (varMatches) {
                    varMatches.forEach(match => {
                        const varName = match.match(/let\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+be/)[1];
                        definedVars.add(varName);
                    });
                }

                // Add functions defined in the document
                const funcMatches = text.match(/def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+(with|do)/g);
                if (funcMatches) {
                    funcMatches.forEach(match => {
                        const funcName = match.match(/def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+(with|do)/)[1];
                        definedVars.add(funcName);
                    });
                }

                // Add keywords
                FLOWLANG_KEYWORDS.forEach(keyword => definedVars.add(keyword));
                FLOWLANG_FUNCTIONS.forEach(func => definedVars.add(func.name));
                builtinFunctions.forEach(func => definedVars.add(func));
                stdlibFunctions.forEach(func => definedVars.add(func));

                // FlowLang syntax validation
                const documentVars = new Set(definedVars);
                const documentFunctions = new Set();
                
                // First pass: collect all variable and function definitions
                const allLines = text.split('\n');
                for (let j = 0; j < allLines.length; j++) {
                    const currentLine = allLines[j].trim();
                    
                        // Variable definitions: let varname be ...
                    const letMatch = currentLine.match(/^let\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+be/);
                    if (letMatch) {
                        documentVars.add(letMatch[1]);
                    }
                    
                    // Function definitions: def funcname with ... do
                    const defMatch = currentLine.match(/^def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+with/);
                    if (defMatch) {
                        documentFunctions.add(defMatch[1]);
                        documentVars.add(defMatch[1]);
                    }
                    
                    // Function definitions without parameters: def funcname do
                    const defNoParamsMatch = currentLine.match(/^def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s+do/);
                    if (defNoParamsMatch) {
                        documentFunctions.add(defNoParamsMatch[1]);
                        documentVars.add(defNoParamsMatch[1]);
                    }
                }

                variableUsage.forEach(variable => {
                    if (!documentVars.has(variable) && !line.startsWith('let ' + variable) && !line.startsWith('def ' + variable)) {
                        const varIndex = line.indexOf(variable);
                        if (varIndex !== -1) {
                            const diagnostic = new vscode.Diagnostic(
                                new vscode.Range(lineNumber, varIndex, lineNumber, varIndex + variable.length),
                                `Undefined variable or function: '${variable}'`,
                                vscode.DiagnosticSeverity.Warning
                            );
                            diagnostic.code = 'undefined-variable';
                            diagnostics.push(diagnostic);
                        }
                    }
                });
            }
        }

        // Check for structural issues
        const blockKeywords = ['if', 'while', 'for', 'def'];
        const endKeywords = ['end'];
        let blockStack = [];

        for (let i = 0; i < lines.length; i++) {
            const line = lines[i].trim();
            
            blockKeywords.forEach(keyword => {
                if (line.startsWith(keyword + ' ')) {
                    blockStack.push({ keyword, line: i });
                }
            });

            if (line === 'end') {
                if (blockStack.length === 0) {
                    const diagnostic = new vscode.Diagnostic(
                        new vscode.Range(i, 0, i, line.length),
                        'Unexpected "end" keyword - no matching block statement',
                        vscode.DiagnosticSeverity.Error
                    );
                    diagnostic.code = 'unexpected-end';
                    diagnostics.push(diagnostic);
                } else {
                    blockStack.pop();
                }
            }
        }

        // Check for unclosed blocks
        blockStack.forEach(block => {
            const diagnostic = new vscode.Diagnostic(
                new vscode.Range(block.line, 0, block.line, lines[block.line].length),
                `Unclosed ${block.keyword} block - missing "end" keyword`,
                vscode.DiagnosticSeverity.Error
            );
            diagnostic.code = 'unclosed-block';
            diagnostics.push(diagnostic);
        });

        return diagnostics.slice(0, config.get('maxNumberOfProblems', 100));
    }
}

async function checkSyntaxWithInterpreter(document) {
    return new Promise((resolve) => {
        const config = vscode.workspace.getConfiguration('flowlang');
        const interpreterPath = config.get('interpreterPath', 'flowlang');
        
        // Create a temporary file
        const tempFile = path.join(__dirname, 'temp_syntax_check.flow');
        require('fs').writeFileSync(tempFile, document.getText());

        const process = spawn(interpreterPath, ['--check-syntax', tempFile]);
        let output = '';
        let errorOutput = '';

        process.stdout.on('data', (data) => {
            output += data.toString();
        });

        process.stderr.on('data', (data) => {
            errorOutput += data.toString();
        });

        process.on('close', (code) => {
            // Clean up temp file
            try {
                require('fs').unlinkSync(tempFile);
            } catch (e) {
                // Ignore cleanup errors
            }

            const diagnostics = [];
            if (code !== 0 && errorOutput) {
                // Parse error output for line numbers and messages
                const errorLines = errorOutput.split('\n');
                errorLines.forEach(errorLine => {
                    const match = errorLine.match(/line (\d+):(.*)/i);
                    if (match) {
                        const lineNumber = parseInt(match[1]) - 1;
                        const message = match[2].trim();
                        const diagnostic = new vscode.Diagnostic(
                            new vscode.Range(lineNumber, 0, lineNumber, document.lineAt(lineNumber).text.length),
                            message,
                            vscode.DiagnosticSeverity.Error
                        );
                        diagnostic.code = 'interpreter-error';
                        diagnostics.push(diagnostic);
                    }
                });
            }
            resolve(diagnostics);
        });

        process.on('error', () => {
            // If interpreter is not available, fall back to basic diagnostics
            resolve([]);
        });
    });
}

async function updateDiagnostics(document) {
    if (document.languageId !== 'flowlang') {
        return;
    }

    try {
        // Combine basic diagnostics with interpreter diagnostics
        const basicDiagnostics = await FlowLangDiagnosticProvider.provideDiagnostics(document);
        const interpreterDiagnostics = await checkSyntaxWithInterpreter(document);
        
        const allDiagnostics = [...basicDiagnostics, ...interpreterDiagnostics];
        diagnosticCollection.set(document.uri, allDiagnostics);
    } catch (error) {
        outputChannel.appendLine(`Error updating diagnostics: ${error.message}`);
    }
}

function activate(context) {
    outputChannel = vscode.window.createOutputChannel('FlowLang');
    outputChannel.appendLine('FlowLang extension activated with advanced features!');

    // Create diagnostic collection
    diagnosticCollection = vscode.languages.createDiagnosticCollection('flowlang');
    context.subscriptions.push(diagnosticCollection);

    // Register completion provider
    const completionProvider = vscode.languages.registerCompletionItemProvider(
        'flowlang',
        new FlowLangCompletionProvider(),
        '.', '(', ' '
    );
    context.subscriptions.push(completionProvider);

    // Register hover provider
    const hoverProvider = vscode.languages.registerHoverProvider(
        'flowlang',
        new FlowLangHoverProvider()
    );
    context.subscriptions.push(hoverProvider);

    // Register commands
    const checkSyntaxCommand = vscode.commands.registerCommand('flowlang.checkSyntax', async () => {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'flowlang') {
            await updateDiagnostics(editor.document);
            vscode.window.showInformationMessage('FlowLang syntax check completed.');
        }
    });
    context.subscriptions.push(checkSyntaxCommand);

    const formatDocumentCommand = vscode.commands.registerCommand('flowlang.formatDocument', () => {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'flowlang') {
            // Basic formatting - this could be enhanced with a proper formatter
            vscode.window.showInformationMessage('FlowLang formatting is not yet implemented.');
        }
    });
    context.subscriptions.push(formatDocumentCommand);

    // Set up document change listeners for real-time diagnostics
    const documentChangeListener = vscode.workspace.onDidChangeTextDocument(async (event) => {
        if (event.document.languageId === 'flowlang') {
            // Debounce the diagnostics update
            clearTimeout(documentChangeListener.timer);
            documentChangeListener.timer = setTimeout(() => {
                updateDiagnostics(event.document);
            }, 500);
        }
    });
    context.subscriptions.push(documentChangeListener);

    // Set up document open listener
    const documentOpenListener = vscode.workspace.onDidOpenTextDocument(async (document) => {
        if (document.languageId === 'flowlang') {
            await updateDiagnostics(document);
        }
    });
    context.subscriptions.push(documentOpenListener);

    // Set up document save listener
    const documentSaveListener = vscode.workspace.onDidSaveTextDocument(async (document) => {
        if (document.languageId === 'flowlang') {
            await updateDiagnostics(document);
        }
    });
    context.subscriptions.push(documentSaveListener);

    // Check all open FlowLang documents on activation
    vscode.workspace.textDocuments.forEach(document => {
        if (document.languageId === 'flowlang') {
            updateDiagnostics(document);
        }
    });

    outputChannel.appendLine('FlowLang extension setup complete with:');
    outputChannel.appendLine('- Real-time error detection and diagnostics');
    outputChannel.appendLine('- Intelligent code completion');
    outputChannel.appendLine('- Hover information for keywords and functions');
    outputChannel.appendLine('- Syntax checking commands');
    outputChannel.appendLine('- Advanced language support');
}

function deactivate() {
    if (diagnosticCollection) {
        diagnosticCollection.dispose();
    }
    if (outputChannel) {
        outputChannel.dispose();
    }
    console.log('FlowLang extension deactivated.');
}

module.exports = {
    activate,
    deactivate
};