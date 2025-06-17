# FlowLang v0.1.0-beta Release Notes

🎉 **Welcome to the first beta release of FlowLang!**

FlowLang is a modern, human-friendly, and secure programming language designed to make software development easy for everyone — from absolute beginners to expert developers.

## 🚀 What's New in v0.1.0-beta

### Core Language Features
- ✅ **Complete syntax implementation** with natural, human-friendly keywords
- ✅ **Interactive REPL** for testing code snippets instantly
- ✅ **Memory-safe execution** built with Rust
- ✅ **Cross-platform support** for macOS, Linux, and Windows
- ✅ **Comprehensive error handling** with helpful error messages

### Language Constructs
- Variables and basic data types (strings, numbers, booleans)
- Functions with parameters and return values
- Control flow (if/else, loops)
- Arrays and objects
- Comments and documentation
- Built-in functions (`show`, arithmetic operations)

### Developer Experience
- **VS Code Extension** with syntax highlighting and auto-completion
- **Sublime Text** syntax support
- **Vim/Neovim** syntax highlighting
- **Automatic installation scripts** for all platforms
- **Comprehensive examples** and documentation

## 📦 Installation

### Quick Install (Recommended)

1. **Download the release package:**
   ```bash
   # Download from GitHub releases
   wget https://github.com/flowlang/flowlang/releases/download/v0.1.0-beta/flowlang-0.1.0-beta.tar.gz
   tar -xzf flowlang-0.1.0-beta.tar.gz
   cd flowlang-0.1.0-beta
   ```

2. **Install FlowLang:**
   ```bash
   # Unix/Linux/macOS
   sudo ./install.sh
   
   # Windows
   install.bat
   ```

3. **Install Editor Support:**
   ```bash
   cd editor-support
   ./install.sh
   ```

4. **Verify Installation:**
   ```bash
   flowlang --version
   flowlang examples/hello.flow
   ```

### Alternative Installation Methods

#### Build from Source
```bash
git clone https://github.com/flowlang/flowlang.git
cd flowlang
cargo build --release
sudo cp target/release/flowlang /usr/local/bin/
```

#### Manual Installation
1. Extract the release archive
2. Copy the appropriate binary from `bin/` to your PATH:
   - macOS: `flowlang-macos-x86_64` or `flowlang-darwin-x86_64`
   - Linux: `flowlang-linux-x86_64`
   - Windows: `flowlang-windows-x86_64.exe`

## 🎯 Platform Support

| Platform | Architecture | Status | Binary Name |
|----------|-------------|--------|-------------|
| macOS | x86_64 | ✅ Supported | `flowlang-macos-x86_64` |
| macOS | ARM64 | 🚧 Planned | `flowlang-macos-arm64` |
| Linux | x86_64 | 🚧 Cross-compile | `flowlang-linux-x86_64` |
| Windows | x86_64 | 🚧 Cross-compile | `flowlang-windows-x86_64.exe` |

**Note:** Cross-compilation targets are available but require additional setup. The current release includes native macOS binaries.

## 📝 Usage Examples

### Hello World
```flowlang
# hello.flow
def main do
    show "Hello, FlowLang!"
end

main()
```

### Variables and Functions
```flowlang
# calculator.flow
def add with a, b do
    return a + b
end

def multiply with a, b do
    return a * b
end

let x = 10
let y = 5
let sum = add(x, y)
let product = multiply(x, y)

show "Sum: " + sum
show "Product: " + product
```

### Control Flow
```flowlang
# fibonacci.flow
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

## 🎨 Editor Support

### Visual Studio Code
**Automatic Installation:**
```bash
cd editor-support
./install.sh  # Select option 1
```

**Manual Installation:**
```bash
code --install-extension editor-support/vscode/flowlang-syntax-1.0.0.vsix
```

**Features:**
- ✅ Syntax highlighting
- ✅ Auto-indentation
- ✅ Bracket matching
- ✅ File recognition (`.flow` files)
- ✅ Theme compatibility

### Other Editors
- **Sublime Text**: Automatic installation via `./install.sh`
- **Vim/Neovim**: Syntax files included
- **Global Formatter**: Python-based code formatter

## 🧪 Testing the Installation

### Run the REPL
```bash
flowlang --repl
# or
flowlang -r
```

### Execute Example Programs
```bash
# Basic examples
flowlang examples/hello.flow
flowlang examples/fibonacci.flow
flowlang examples/calculator.flow
flowlang examples/fizzbuzz.flow

# Advanced examples
flowlang examples/collections.flow
flowlang examples/stdlib_test.flow
```

### Command Line Options
```bash
flowlang --help              # Show help
flowlang --version           # Show version
flowlang --verbose file.flow # Run with verbose output
flowlang --ast file.flow     # Show AST (debug)
flowlang --bytecode file.flow # Show bytecode (debug)
```

## 🐛 Known Issues

1. **Cross-compilation**: Linux and Windows binaries require additional Rust targets
2. **ARM64 macOS**: Native ARM64 build requires `aarch64-apple-darwin` target
3. **Package Manager**: Module system and package management not yet implemented
4. **Standard Library**: Limited built-in functions (expanding in future releases)

## 🛣️ Roadmap

### v0.2.0 (Next Release)
- [ ] ARM64 macOS native support
- [ ] Linux and Windows native binaries
- [ ] Expanded standard library
- [ ] Module system and imports
- [ ] Language Server Protocol (LSP) support

### Future Releases
- [ ] Package manager
- [ ] Debugger integration
- [ ] WebAssembly compilation target
- [ ] More editor integrations
- [ ] Performance optimizations

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
1. Fork the repository
2. Clone: `git clone https://github.com/yourusername/flowlang.git`
3. Build: `cargo build --release`
4. Test: `cargo test`
5. Submit a Pull Request

## 📞 Support

- **Documentation**: [docs/](docs/)
- **GitHub Issues**: [Report bugs and request features](https://github.com/flowlang/flowlang/issues)
- **GitHub Discussions**: [Community discussions](https://github.com/flowlang/flowlang/discussions)

## 📄 License

MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

Thanks to all contributors and the Rust community for making this project possible!

---

**FlowLang v0.1.0-beta** - A programming language designed for humans ❤️

*Released on June 17, 2024*