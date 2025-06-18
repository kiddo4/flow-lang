# FlowLang v0.1.0-beta - Beta Release

A modern, human-friendly programming language designed for everyone.

## Quick Start

### Installation

**Unix/Linux/macOS:**
```bash
./install.sh
```

**Windows:**
```cmd
install.bat
```

### Manual Installation

1. Choose the appropriate binary from the `bin/` directory:
   - `flowlang-macos-x86_64` - macOS Intel
   - `flowlang-macos-arm64` - macOS Apple Silicon
   - `flowlang-linux-x86_64` - Linux 64-bit
   - `flowlang-windows-x86_64.exe` - Windows 64-bit

2. Copy it to a directory in your PATH
3. Rename to `flowlang` (or `flowlang.exe` on Windows)
4. Make it executable (Unix/Linux/macOS): `chmod +x flowlang`

## Usage

```bash
# Run a FlowLang program
flowlang program.flow

# Start interactive REPL
flowlang --repl

# Show help
flowlang --help
```

## Editor Support

Syntax highlighting and formatting support is available for:
- Visual Studio Code
- Sublime Text
- Vim/Neovim

**Installation:**
```bash
cd editor-support
./install.sh
```

For VS Code, you can also install the VSIX package:
```bash
code --install-extension editor-support/vscode/flowlang-syntax-1.0.0.vsix
```

## Examples

Check the `examples/` directory for sample FlowLang programs:

- `hello.flow` - Hello World
- `fibonacci.flow` - Fibonacci sequence
- `calculator.flow` - Simple calculator
- And more...

## Language Syntax

```flowlang
# Variables
let name = "FlowLang"
let version = 0.1

# Functions
def greet with name do
    show "Hello, " + name + "!"
end

# Conditionals
if version > 0 then
    show "Beta version"
else
    show "Development version"
end

# Function calls
greet("Developer")
```

## Documentation

See `docs/` directory for detailed documentation.

## Support

- GitHub: https://github.com/flowlang/flowlang
- Issues: https://github.com/flowlang/flowlang/issues

## License

MIT License - see LICENSE file for details.
