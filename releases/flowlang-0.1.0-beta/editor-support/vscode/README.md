# FlowLang Syntax Highlighting

Syntax highlighting and language support for the FlowLang programming language in Visual Studio Code.

## Features

- **Syntax Highlighting**: Full syntax highlighting for FlowLang code
- **Language Recognition**: Automatic detection of `.flow` files
- **Code Formatting**: Proper indentation and bracket matching
- **Theme Compatibility**: Works with all VS Code themes using standard TextMate scopes

## Installation

### Method 1: Install from VSIX (Recommended)

1. Download the `flowlang-syntax-1.0.0.vsix` file
2. Open VS Code
3. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS) to open the command palette
4. Type "Extensions: Install from VSIX" and select it
5. Browse and select the downloaded `.vsix` file
6. Restart VS Code

### Method 2: Manual Installation

1. Copy the extension folder to your VS Code extensions directory:
   - **Windows**: `%USERPROFILE%\.vscode\extensions\flowlang-syntax`
   - **macOS**: `~/.vscode/extensions/flowlang-syntax`
   - **Linux**: `~/.vscode/extensions/flowlang-syntax`
2. Restart VS Code

## Usage

1. Open any `.flow` file in VS Code
2. The syntax highlighting should automatically activate
3. If not, you can manually set the language mode:
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)
   - Type "Change Language Mode"
   - Select "FlowLang"

## FlowLang Syntax Examples

```flowlang
# Function definition
def fibonacci with n do
    if n <= 1 then
        return n
    else
        return fibonacci(n-1) + fibonacci(n-2)
    end
end

# Variable assignment
let result = fibonacci(10)
show result

# Conditional statements
if result > 50 then
    show "Large number!"
else
    show "Small number"
end
```

## Supported Language Features

- Keywords: `def`, `end`, `if`, `then`, `else`, `let`, `show`, `return`
- Operators: `+`, `-`, `*`, `/`, `=`, `==`, `!=`, `<`, `>`, `<=`, `>=`
- Comments: Lines starting with `#`
- Strings: Double-quoted strings with escape sequences
- Numbers: Integer and floating-point literals
- Functions: Function definitions and calls
- Variables: Variable declarations and references

## Troubleshooting

### Syntax highlighting not working?

1. **Check file extension**: Make sure your file has the `.flow` extension
2. **Restart VS Code**: Close and reopen VS Code completely
3. **Manual language selection**: Use `Ctrl+Shift+P` → "Change Language Mode" → "FlowLang"
4. **Check installation**: Verify the extension appears in the Extensions panel

### Extension not appearing in Extensions list?

1. **Restart VS Code**: The extension may need a restart to be recognized
2. **Check installation path**: Ensure files are in the correct extensions directory
3. **Reload window**: Use `Ctrl+Shift+P` → "Developer: Reload Window"

## Contributing

If you find issues or want to contribute improvements:

1. Report bugs or request features on our [GitHub repository](https://github.com/flowlang/flowlang/issues)
2. Submit pull requests for improvements
3. Help improve the syntax highlighting grammar

## License

MIT License - see the LICENSE file for details.

## Changelog

### 1.0.0
- Initial release
- Full syntax highlighting support
- Language configuration for proper indentation
- Theme compatibility with standard TextMate scopes
- Support for all FlowLang language features