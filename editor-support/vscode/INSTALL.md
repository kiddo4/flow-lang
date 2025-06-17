# FlowLang VS Code Extension - Global Installation Guide

This guide explains how to install the FlowLang syntax highlighting extension globally in VS Code so it works across all instances and projects.

## Method 1: Install via VSIX Package (Recommended)

The VSIX package allows for global installation that works across all VS Code instances.

### Prerequisites
- Visual Studio Code installed
- VS Code CLI (`code` command) available in your PATH

### Installation Steps

1. **Download the VSIX package**:
   - Get the `flowlang-syntax-1.0.0.vsix` file from this directory

2. **Install via Command Line** (Recommended):
   ```bash
   code --install-extension flowlang-syntax-1.0.0.vsix
   ```

3. **Install via VS Code GUI**:
   - Open VS Code
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)
   - Type "Extensions: Install from VSIX"
   - Select the command and browse to the `.vsix` file
   - Click "Install"

4. **Restart VS Code** to activate the extension

### Verification

1. Open any `.flow` file or create a new file with `.flow` extension
2. The syntax highlighting should automatically activate
3. Check the language mode in the bottom-right corner - it should show "FlowLang"

## Method 2: Automatic Installation via Script

Use the provided installation script which will automatically detect VS Code and install the extension:

```bash
# From the project root directory
./editor-support/install.sh
# Select option 1 for VS Code
```

The script will:
1. Detect if VS Code CLI is available
2. Install via VSIX package if possible (global installation)
3. Fall back to manual installation if needed

## Method 3: Manual Installation

If the above methods don't work, you can manually install the extension:

### Steps

1. **Locate your VS Code extensions directory**:
   - **Windows**: `%USERPROFILE%\.vscode\extensions`
   - **macOS**: `~/.vscode/extensions`
   - **Linux**: `~/.vscode/extensions`

2. **Create the extension directory**:
   ```bash
   mkdir -p ~/.vscode/extensions/flowlang-syntax
   ```

3. **Copy extension files**:
   ```bash
   cp -r /path/to/flowlang/editor-support/vscode/* ~/.vscode/extensions/flowlang-syntax/
   ```

4. **Restart VS Code**

## Troubleshooting

### Extension not working?

1. **Check installation**:
   ```bash
   code --list-extensions | grep flowlang
   ```
   You should see `flowlang.flowlang-syntax` in the output.

2. **Force reload VS Code**:
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P`)
   - Type "Developer: Reload Window"
   - Select the command

3. **Manual language selection**:
   - Open a `.flow` file
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P`)
   - Type "Change Language Mode"
   - Select "FlowLang"

4. **Check for conflicts**:
   - Disable other language extensions that might conflict
   - Check if file associations are correct

### VS Code CLI not available?

If the `code` command is not available:

1. **Windows**: Add VS Code to PATH during installation
2. **macOS**: 
   - Open VS Code
   - Press `Cmd+Shift+P`
   - Type "Shell Command: Install 'code' command in PATH"
   - Select the command
3. **Linux**: Usually available by default, or install via package manager

## Features After Installation

Once installed globally, the extension provides:

- ✅ **Automatic recognition** of `.flow` files
- ✅ **Syntax highlighting** for all FlowLang constructs
- ✅ **Proper indentation** and code formatting
- ✅ **Bracket matching** and auto-completion
- ✅ **Theme compatibility** with all VS Code themes
- ✅ **Global availability** across all VS Code instances

## Uninstallation

To remove the extension:

### Via Command Line:
```bash
code --uninstall-extension flowlang.flowlang-syntax
```

### Via VS Code GUI:
1. Open Extensions panel (`Ctrl+Shift+X`)
2. Search for "FlowLang"
3. Click the gear icon and select "Uninstall"

### Manual Removal:
```bash
rm -rf ~/.vscode/extensions/flowlang-syntax
```

## Support

If you encounter issues:

1. Check this troubleshooting guide
2. Report issues on [GitHub](https://github.com/flowlang/flowlang/issues)
3. Include your VS Code version and operating system in bug reports