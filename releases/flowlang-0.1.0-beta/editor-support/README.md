# FlowLang Editor Support

This directory contains syntax highlighting, code formatting, and editor integration files for FlowLang. These tools will help your development team work more effectively with FlowLang code.

## Features

- **Syntax Highlighting**: Proper color coding for FlowLang keywords, strings, numbers, comments, and operators
- **Code Formatting**: Automatic indentation and code style formatting
- **Editor Integration**: Support for popular editors including VS Code, Sublime Text, and Vim
- **Language Configuration**: Auto-closing brackets, comment toggling, and smart indentation

## Supported Editors

### Visual Studio Code

The most comprehensive support with syntax highlighting, auto-completion, and proper language configuration.

**Installation:**
1. Copy the `vscode` folder to your VS Code extensions directory:
   - **macOS**: `~/.vscode/extensions/flowlang-syntax/`
   - **Linux**: `~/.vscode/extensions/flowlang-syntax/`
   - **Windows**: `%USERPROFILE%\.vscode\extensions\flowlang-syntax\`

2. Restart VS Code
3. Open any `.flow` file to see syntax highlighting

**Features:**
- Syntax highlighting for all FlowLang constructs
- Auto-closing brackets and quotes
- Comment toggling with `Cmd+/` (Mac) or `Ctrl+/` (Windows/Linux)
- Smart indentation for `do`/`end` and `if`/`then`/`else` blocks

### Sublime Text

**Installation:**
1. Copy `sublime-text/FlowLang.sublime-syntax` to your Sublime Text packages directory:
   - **macOS**: `~/Library/Application Support/Sublime Text/Packages/User/`
   - **Linux**: `~/.config/sublime-text/Packages/User/`
   - **Windows**: `%APPDATA%\Sublime Text\Packages\User\`

2. Restart Sublime Text
3. Open any `.flow` file - syntax should be automatically detected

### Vim/Neovim

**Installation:**
1. Copy the vim files to your vim configuration directory:
   ```bash
   # For Vim
   cp vim/syntax/flowlang.vim ~/.vim/syntax/
   cp vim/ftdetect/flowlang.vim ~/.vim/ftdetect/
   
   # For Neovim
   cp vim/syntax/flowlang.vim ~/.config/nvim/syntax/
   cp vim/ftdetect/flowlang.vim ~/.config/nvim/ftdetect/
   ```

2. Restart Vim/Neovim
3. Open any `.flow` file to see syntax highlighting

## Code Formatter

The `flowfmt.py` script provides automatic code formatting for FlowLang files.

### Installation

```bash
# Make the formatter executable (already done)
chmod +x formatter/flowfmt.py

# Optionally, add to your PATH for global access
sudo ln -s $(pwd)/formatter/flowfmt.py /usr/local/bin/flowfmt
```

### Usage

```bash
# Format a file in-place
python formatter/flowfmt.py examples/hello.flow

# Check if a file needs formatting (useful for CI)
python formatter/flowfmt.py --check examples/hello.flow

# Format from stdin
cat examples/hello.flow | python formatter/flowfmt.py --stdin

# Format to a different file
python formatter/flowfmt.py examples/hello.flow -o formatted_hello.flow
```

### Formatter Features

- **Smart Indentation**: Automatically indents code blocks (`do`/`end`, `if`/`then`/`else`)
- **Consistent Spacing**: Adds proper spacing around operators
- **Preserves Comments**: Maintains comment formatting and positioning
- **CI Integration**: `--check` flag for continuous integration workflows

## Integration with Development Workflow

### Pre-commit Hook

Add FlowLang formatting to your git pre-commit hooks:

```bash
#!/bin/sh
# .git/hooks/pre-commit

# Format all FlowLang files
for file in $(git diff --cached --name-only --diff-filter=ACM | grep '\.flow$'); do
    python editor-support/formatter/flowfmt.py "$file"
    git add "$file"
done
```

### VS Code Integration

For automatic formatting on save in VS Code, add to your `settings.json`:

```json
{
    "[flowlang]": {
        "editor.formatOnSave": true,
        "editor.defaultFormatter": "flowlang.flowlang-syntax"
    }
}
```

## FlowLang Syntax Reference

For editor configuration reference, here are the key FlowLang language constructs:

### Keywords
- **Control Flow**: `if`, `then`, `else`, `end`, `while`, `for`, `from`, `to`, `do`
- **Functions**: `def`, `with`, `return`
- **Variables**: `let`, `be`
- **Operators**: `and`, `or`, `not`
- **Built-ins**: `show`
- **Constants**: `true`, `false`, `null`

### Syntax Patterns
- **Comments**: `# This is a comment`
- **Variables**: `let name be "value"`
- **Functions**: `def functionName with param1, param2 do ... end`
- **Conditionals**: `if condition then ... else ... end`
- **Loops**: `for i from 1 to 10 do ... end`
- **Arrays**: `[1, 2, 3, 4, 5]`
- **Objects**: `{"key": "value", "number": 42}`

## Testing the Setup

To test that everything is working correctly:

1. Open any `.flow` file from the `examples/` directory
2. Verify that syntax highlighting is working (keywords should be colored)
3. Test the formatter:
   ```bash
   python editor-support/formatter/flowfmt.py examples/hello.flow --check
   ```
4. Try auto-completion and bracket matching in your editor

## Troubleshooting

### VS Code
- If syntax highlighting isn't working, check that the extension is in the correct directory
- Reload the window with `Cmd+Shift+P` → "Developer: Reload Window"

### Sublime Text
- If syntax isn't detected, manually set it via `View` → `Syntax` → `FlowLang`
- Check that the file is in the correct Packages directory

### Vim
- If syntax highlighting isn't working, try `:set filetype=flowlang` manually
- Check that files are in the correct vim directories

### Formatter
- Ensure Python 3 is installed and accessible
- Check file permissions if the script won't execute

## Contributing

To improve editor support:

1. Test with various FlowLang code samples
2. Report issues with syntax highlighting edge cases
3. Suggest improvements for the formatter
4. Add support for additional editors

The syntax definitions are designed to be easily extensible as FlowLang evolves.