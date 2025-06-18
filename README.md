# FlowLang

A modern, human-friendly, and secure programming language designed to make software development easy for everyone — from absolute beginners to expert developers.

## Features

- **Easy to Learn**: Natural syntax that feels like human thinking
- **Memory Safe**: Built with Rust for security and performance
- **Human-Friendly**: Clean, minimal syntax inspired by natural language
- **Interactive REPL**: Test code snippets instantly
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Editor Support**: Syntax highlighting for VS Code, Sublime Text, Vim/Neovim

## Quick Start

### Download Pre-built Release (Recommended)

1. **Download the latest release**:
   - Go to [Releases](https://github.com/kiddo4/flowlang/releases)
   - Download `flowlang-0.1.0-beta.tar.gz` or `flowlang-0.1.0-beta.zip`
   - Extract the archive

2. **Install FlowLang**:
   ```bash
   # Unix/Linux/macOS
   cd flowlang-0.1.0-beta
   ./install.sh
   
   # Windows
   install.bat
   ```

3. **Install Editor Support**:
   ```bash
   cd editor-support
   ./install.sh
   ```

4. **Test Installation**:
   ```bash
   flowlang --help
   flowlang examples/hello.flow
   ```

### Build from Source

#### Prerequisites
- Rust 1.70+ installed on your system
- Git

#### Build Steps

```bash
git clone https://github.com/flowlang/flowlang.git
cd flowlang
cargo build --release
```

The compiled binary will be available at `target/release/flowlang`.

#### Install Globally

```bash
# Copy to system PATH
sudo cp target/release/flowlang /usr/local/bin/
# or for user-only installation
cp target/release/flowlang ~/.local/bin/
```

## Usage

### Running FlowLang Files

```bash
# Run a FlowLang program
flowlang program.flow

# Run with verbose output
flowlang --verbose program.flow
```

### Interactive REPL

```bash
# Start the interactive REPL
flowlang --repl
# or
flowlang -r
```

### Command Line Options

```bash
flowlang --help              # Show help
flowlang --version           # Show version
flowlang --repl              # Start REPL
flowlang --verbose file.flow # Run with verbose output
flowlang --ast file.flow     # Show AST (debug)
flowlang --bytecode file.flow # Show bytecode (debug)
```

## Language Syntax

### Variables and Basic Types

```flowlang
# Variables
let name be "FlowLang"
let version be 0.1
let is_beta be true

# Arrays
let numbers be [1, 2, 3, 4, 5]
let mixed be ["hello", 42, true]

# Objects
let person be {
    name: "Alice",
    age: 30,
    active: true
}
```

### Functions

```flowlang
# Function definition
def greet with name do
    show "Hello, " + name + "!"
end

# Function with return value
def add with a, b do
    return a + b
end

# Function calls
greet("Developer")
let result = add(5, 3)
```

### Control Flow

```flowlang
# Conditionals
if version > 0 then
    show "Beta version"
else
    show "Development version"
end

# Loops
for i in 1 to 10 do
    show i
end

for item in numbers do
    show item
end
```

### Comments

```flowlang
# This is a single-line comment

# Multi-line comments are just
# multiple single-line comments
```

## Editor Support

FlowLang provides syntax highlighting and formatting support for popular editors:

### Visual Studio Code

**Option 1: Install via VSIX (Recommended)**
```bash
code --install-extension editor-support/vscode/flowlang-syntax-1.0.0.vsix
```

**Option 2: Automatic Installation**
```bash
cd editor-support
./install.sh
# Select option 1 for VS Code
```

### Sublime Text

```bash
cd editor-support
./install.sh
# Select option 2 for Sublime Text
```

### Vim/Neovim

```bash
cd editor-support
./install.sh
# Select option 3 for Vim or option 4 for Neovim
```

### Features Provided

- ✅ **Syntax Highlighting**: Keywords, strings, numbers, comments
- ✅ **Auto-indentation**: Proper code formatting
- ✅ **Bracket Matching**: Automatic bracket completion
- ✅ **File Recognition**: Automatic detection of `.flow` files
- ✅ **Theme Compatibility**: Works with all editor themes

## Examples

The `examples/` directory contains sample FlowLang programs:

- `hello.flow` - Hello World program
- `fibonacci.flow` - Fibonacci sequence generator
- `calculator.flow` - Simple calculator
- `fizzbuzz.flow` - FizzBuzz implementation
- `collections.flow` - Working with arrays and objects

### Hello World

```flowlang
# examples/hello.flow
def main do
    show "Hello, FlowLang!"
end

main()
```

### Fibonacci Sequence

```flowlang
# examples/fibonacci.flow
def fibonacci with n do
    if n <= 1 then
        return n
    else
        return fibonacci(n-1) + fibonacci(n-2)
    end
end

for i in 1 to 10 do
    show "fib(" + i + ") = " + fibonacci(i)
end
```

## Development

### Project Structure

```
flowlang/
├── src/                 # Rust source code
│   ├── main.rs         # CLI entry point
│   ├── lib.rs          # Library entry point
│   ├── lexer.rs        # Tokenizer
│   ├── parser.rs       # Parser
│   ├── ast.rs          # Abstract Syntax Tree
│   ├── interpreter.rs  # Interpreter
│   └── stdlib.rs       # Standard library
├── examples/           # Example FlowLang programs
├── editor-support/     # Editor integrations
│   ├── vscode/        # VS Code extension
│   ├── sublime-text/  # Sublime Text syntax
│   └── vim/           # Vim syntax
├── docs/              # Documentation
└── tests/             # Test suite
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Building Release

```bash
# Build optimized release
cargo build --release

# Create cross-platform release package
./scripts/build-release.sh
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/flowlang.git`
3. Create a feature branch: `git checkout -b feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit and push: `git commit -am "Add feature" && git push`
7. Create a Pull Request

## License

MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [docs/](docs/)
- **Issues**: [GitHub Issues](https://github.com/flowlang/flowlang/issues)
- **Discussions**: [GitHub Discussions](https://github.com/flowlang/flowlang/discussions)

## Roadmap

- [ ] Package manager and module system
- [ ] Standard library expansion
- [ ] Language server protocol (LSP) support
- [ ] Debugger integration
- [ ] WebAssembly compilation target
- [ ] More editor integrations

---

**FlowLang v0.1.0-beta** - A programming language designed for humans ❤️