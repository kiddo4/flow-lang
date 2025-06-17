#!/bin/bash

# FlowLang Release Builder
# Builds cross-platform binaries and creates release packages

set -e

VERSION="0.1.0-beta"
RELEASE_DIR="releases/flowlang-${VERSION}"
BIN_DIR="${RELEASE_DIR}/bin"
DOCS_DIR="${RELEASE_DIR}/docs"
EDITOR_DIR="${RELEASE_DIR}/editor-support"
EXAMPLES_DIR="${RELEASE_DIR}/examples"

echo "🚀 Building FlowLang Release v${VERSION}"
echo "=========================================="

# Clean previous releases
rm -rf releases/
mkdir -p "$BIN_DIR" "$DOCS_DIR" "$EDITOR_DIR" "$EXAMPLES_DIR"

# Build for current platform (development build)
echo "📦 Building for current platform..."
cargo build --release
cp target/release/flowlang "$BIN_DIR/flowlang-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)"

# Cross-compilation targets (if available)
echo "🔧 Attempting cross-compilation..."

# macOS (if on macOS or with cross-compilation setup)
if [[ "$(uname)" == "Darwin" ]]; then
    echo "  Building for macOS (x86_64)..."
    cargo build --release --target x86_64-apple-darwin 2>/dev/null || echo "    ⚠️  x86_64-apple-darwin target not available"
    if [[ -f "target/x86_64-apple-darwin/release/flowlang" ]]; then
        cp target/x86_64-apple-darwin/release/flowlang "$BIN_DIR/flowlang-macos-x86_64"
    fi
    
    echo "  Building for macOS (ARM64)..."
    cargo build --release --target aarch64-apple-darwin 2>/dev/null || echo "    ⚠️  aarch64-apple-darwin target not available"
    if [[ -f "target/aarch64-apple-darwin/release/flowlang" ]]; then
        cp target/aarch64-apple-darwin/release/flowlang "$BIN_DIR/flowlang-macos-arm64"
    fi
fi

# Linux (if cross-compilation is set up)
echo "  Attempting Linux builds..."
cargo build --release --target x86_64-unknown-linux-gnu 2>/dev/null || echo "    ⚠️  x86_64-unknown-linux-gnu target not available"
if [[ -f "target/x86_64-unknown-linux-gnu/release/flowlang" ]]; then
    cp target/x86_64-unknown-linux-gnu/release/flowlang "$BIN_DIR/flowlang-linux-x86_64"
fi

# Windows (if cross-compilation is set up)
echo "  Attempting Windows build..."
cargo build --release --target x86_64-pc-windows-gnu 2>/dev/null || echo "    ⚠️  x86_64-pc-windows-gnu target not available"
if [[ -f "target/x86_64-pc-windows-gnu/release/flowlang.exe" ]]; then
    cp target/x86_64-pc-windows-gnu/release/flowlang.exe "$BIN_DIR/flowlang-windows-x86_64.exe"
fi

# Copy documentation
echo "📚 Copying documentation..."
cp README.md "$DOCS_DIR/"
cp -r docs/* "$DOCS_DIR/" 2>/dev/null || echo "    No additional docs found"

# Copy examples
echo "📝 Copying examples..."
cp examples/*.flow "$EXAMPLES_DIR/" 2>/dev/null || echo "    No examples found"

# Copy editor support
echo "🎨 Copying editor support..."
cp -r editor-support/* "$EDITOR_DIR/"

# Create installation scripts
echo "📋 Creating installation scripts..."

# Unix installation script
cat > "$RELEASE_DIR/install.sh" << 'EOF'
#!/bin/bash

# FlowLang Installation Script

set -e

echo "🚀 Installing FlowLang..."

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case $ARCH in
    x86_64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="arm64" ;;
    *) echo "❌ Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Find the right binary
BINARY=""
case $OS in
    darwin)
        if [[ -f "bin/flowlang-macos-$ARCH" ]]; then
            BINARY="bin/flowlang-macos-$ARCH"
        elif [[ -f "bin/flowlang-macos-x86_64" ]]; then
            BINARY="bin/flowlang-macos-x86_64"
        fi
        ;;
    linux)
        if [[ -f "bin/flowlang-linux-$ARCH" ]]; then
            BINARY="bin/flowlang-linux-$ARCH"
        elif [[ -f "bin/flowlang-linux-x86_64" ]]; then
            BINARY="bin/flowlang-linux-x86_64"
        fi
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

if [[ -z "$BINARY" ]]; then
    echo "❌ No compatible binary found for $OS-$ARCH"
    echo "Available binaries:"
    ls -la bin/
    exit 1
fi

echo "📦 Found binary: $BINARY"

# Install to /usr/local/bin if possible, otherwise ~/bin
if [[ -w "/usr/local/bin" ]]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="$HOME/bin"
    mkdir -p "$INSTALL_DIR"
fi

echo "📁 Installing to $INSTALL_DIR/flowlang"
cp "$BINARY" "$INSTALL_DIR/flowlang"
chmod +x "$INSTALL_DIR/flowlang"

echo "✅ FlowLang installed successfully!"
echo "📍 Binary location: $INSTALL_DIR/flowlang"

# Check if directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "⚠️  $INSTALL_DIR is not in your PATH"
    echo "   Add this line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo "   export PATH=\"$INSTALL_DIR:\$PATH\""
fi

echo ""
echo "🎉 Installation complete!"
echo "   Run 'flowlang --help' to get started"
echo "   Run 'flowlang --repl' for interactive mode"
echo ""
echo "📚 Editor support available in: editor-support/"
echo "   Run: ./editor-support/install.sh"
EOF

# Windows installation script
cat > "$RELEASE_DIR/install.bat" << 'EOF'
@echo off
echo Installing FlowLang for Windows...

if exist "bin\flowlang-windows-x86_64.exe" (
    echo Found Windows binary
    copy "bin\flowlang-windows-x86_64.exe" "%USERPROFILE%\flowlang.exe"
    echo FlowLang installed to %USERPROFILE%\flowlang.exe
    echo.
    echo Add %USERPROFILE% to your PATH to use 'flowlang' command globally
    echo Or run: %USERPROFILE%\flowlang.exe
) else (
    echo Error: Windows binary not found
    echo Available files:
    dir bin
)

pause
EOF

# Make scripts executable
chmod +x "$RELEASE_DIR/install.sh"

# Create README for the release
cat > "$RELEASE_DIR/README.md" << EOF
# FlowLang v${VERSION} - Beta Release

A modern, human-friendly programming language designed for everyone.

## Quick Start

### Installation

**Unix/Linux/macOS:**
\`\`\`bash
./install.sh
\`\`\`

**Windows:**
\`\`\`cmd
install.bat
\`\`\`

### Manual Installation

1. Choose the appropriate binary from the \`bin/\` directory:
   - \`flowlang-macos-x86_64\` - macOS Intel
   - \`flowlang-macos-arm64\` - macOS Apple Silicon
   - \`flowlang-linux-x86_64\` - Linux 64-bit
   - \`flowlang-windows-x86_64.exe\` - Windows 64-bit

2. Copy it to a directory in your PATH
3. Rename to \`flowlang\` (or \`flowlang.exe\` on Windows)
4. Make it executable (Unix/Linux/macOS): \`chmod +x flowlang\`

## Usage

\`\`\`bash
# Run a FlowLang program
flowlang program.flow

# Start interactive REPL
flowlang --repl

# Show help
flowlang --help
\`\`\`

## Editor Support

Syntax highlighting and formatting support is available for:
- Visual Studio Code
- Sublime Text
- Vim/Neovim

**Installation:**
\`\`\`bash
cd editor-support
./install.sh
\`\`\`

For VS Code, you can also install the VSIX package:
\`\`\`bash
code --install-extension editor-support/vscode/flowlang-syntax-1.0.0.vsix
\`\`\`

## Examples

Check the \`examples/\` directory for sample FlowLang programs:

- \`hello.flow\` - Hello World
- \`fibonacci.flow\` - Fibonacci sequence
- \`calculator.flow\` - Simple calculator
- And more...

## Language Syntax

\`\`\`flowlang
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
\`\`\`

## Documentation

See \`docs/\` directory for detailed documentation.

## Support

- GitHub: https://github.com/flowlang/flowlang
- Issues: https://github.com/flowlang/flowlang/issues

## License

MIT License - see LICENSE file for details.
EOF

# Create archive
echo "📦 Creating release archive..."
cd releases
tar -czf "flowlang-${VERSION}.tar.gz" "flowlang-${VERSION}/"
zip -r "flowlang-${VERSION}.zip" "flowlang-${VERSION}/" >/dev/null 2>&1 || echo "    ⚠️  zip not available"

echo ""
echo "✅ Release build complete!"
echo "📁 Release directory: $RELEASE_DIR"
echo "📦 Archives created:"
echo "   - flowlang-${VERSION}.tar.gz"
echo "   - flowlang-${VERSION}.zip (if zip available)"
echo ""
echo "🎯 Available binaries:"
ls -la "$BIN_DIR" 2>/dev/null || echo "   No binaries found in $BIN_DIR"
echo ""
echo "🚀 Ready for distribution!"