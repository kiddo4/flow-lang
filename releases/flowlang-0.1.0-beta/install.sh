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
