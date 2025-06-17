#!/bin/bash

# FlowLang Editor Support Installation Script
# This script helps install FlowLang syntax highlighting and tools for various editors

set -e

echo "🚀 FlowLang Editor Support Installer"
echo "====================================="
echo

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Function to install VS Code support
install_vscode() {
    echo "📝 Installing VS Code support..."
    
    # Check if code command is available for VSIX installation
    if command -v code &> /dev/null; then
        echo "🔍 VS Code CLI detected, installing via VSIX package..."
        
        # Check if VSIX file exists
        VSIX_FILE="$SCRIPT_DIR/vscode/flowlang-syntax-1.0.0.vsix"
        if [[ -f "$VSIX_FILE" ]]; then
            echo "📦 Installing FlowLang extension globally..."
            code --install-extension "$VSIX_FILE" --force
            
            if [[ $? -eq 0 ]]; then
                echo "✅ FlowLang extension installed globally via VSIX"
                echo "   The extension will work in all VS Code instances"
                echo "   Restart VS Code to activate FlowLang syntax highlighting"
                return 0
            else
                echo "⚠️  VSIX installation failed, falling back to manual installation..."
            fi
        else
            echo "⚠️  VSIX package not found, falling back to manual installation..."
        fi
    else
        echo "⚠️  VS Code CLI not found, using manual installation..."
    fi
    
    # Fallback to manual installation
    echo "📁 Installing via manual copy..."
    
    # Determine VS Code extensions directory
    if [[ "$OSTYPE" == "darwin"* ]]; then
        VSCODE_DIR="$HOME/.vscode/extensions"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        VSCODE_DIR="$HOME/.vscode/extensions"
    elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
        VSCODE_DIR="$USERPROFILE/.vscode/extensions"
    else
        echo "❌ Unsupported OS for VS Code installation"
        return 1
    fi
    
    # Create extensions directory if it doesn't exist
    mkdir -p "$VSCODE_DIR/flowlang-syntax"
    
    # Copy VS Code files (exclude VSIX and build artifacts)
    rsync -av --exclude='*.vsix' --exclude='.vscodeignore' "$SCRIPT_DIR/vscode/" "$VSCODE_DIR/flowlang-syntax/"
    
    echo "✅ VS Code support installed to $VSCODE_DIR/flowlang-syntax"
    echo "   Restart VS Code to activate FlowLang syntax highlighting"
}

# Function to install Sublime Text support
install_sublime() {
    echo "📝 Installing Sublime Text support..."
    
    # Determine Sublime Text packages directory
    if [[ "$OSTYPE" == "darwin"* ]]; then
        SUBLIME_DIR="$HOME/Library/Application Support/Sublime Text/Packages/User"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        SUBLIME_DIR="$HOME/.config/sublime-text/Packages/User"
    elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
        SUBLIME_DIR="$APPDATA/Sublime Text/Packages/User"
    else
        echo "❌ Unsupported OS for Sublime Text installation"
        return 1
    fi
    
    # Create directory if it doesn't exist
    mkdir -p "$SUBLIME_DIR"
    
    # Copy Sublime Text syntax file
    cp "$SCRIPT_DIR/sublime-text/FlowLang.sublime-syntax" "$SUBLIME_DIR/"
    
    echo "✅ Sublime Text support installed to $SUBLIME_DIR"
    echo "   Restart Sublime Text to activate FlowLang syntax highlighting"
}

# Function to install Vim support
install_vim() {
    echo "📝 Installing Vim support..."
    
    # Create vim directories
    mkdir -p "$HOME/.vim/syntax"
    mkdir -p "$HOME/.vim/ftdetect"
    
    # Copy vim files
    cp "$SCRIPT_DIR/vim/syntax/flowlang.vim" "$HOME/.vim/syntax/"
    cp "$SCRIPT_DIR/vim/ftdetect/flowlang.vim" "$HOME/.vim/ftdetect/"
    
    echo "✅ Vim support installed to ~/.vim/"
    echo "   FlowLang syntax highlighting will be active on next Vim start"
}

# Function to install Neovim support
install_neovim() {
    echo "📝 Installing Neovim support..."
    
    # Create neovim directories
    mkdir -p "$HOME/.config/nvim/syntax"
    mkdir -p "$HOME/.config/nvim/ftdetect"
    
    # Copy vim files to neovim directories
    cp "$SCRIPT_DIR/vim/syntax/flowlang.vim" "$HOME/.config/nvim/syntax/"
    cp "$SCRIPT_DIR/vim/ftdetect/flowlang.vim" "$HOME/.config/nvim/ftdetect/"
    
    echo "✅ Neovim support installed to ~/.config/nvim/"
    echo "   FlowLang syntax highlighting will be active on next Neovim start"
}

# Function to install formatter globally
install_formatter() {
    echo "🔧 Installing FlowLang formatter..."
    
    # Check if python3 is available
    if ! command -v python3 &> /dev/null; then
        echo "❌ Python 3 is required for the formatter but not found"
        echo "   Please install Python 3 and try again"
        return 1
    fi
    
    # Make formatter executable
    chmod +x "$SCRIPT_DIR/formatter/flowfmt.py"
    
    # Try to install globally
    if [[ -w "/usr/local/bin" ]]; then
        ln -sf "$SCRIPT_DIR/formatter/flowfmt.py" "/usr/local/bin/flowfmt"
        echo "✅ FlowLang formatter installed globally as 'flowfmt'"
    else
        echo "⚠️  Could not install formatter globally (permission denied)"
        echo "   You can run it directly: python3 $SCRIPT_DIR/formatter/flowfmt.py"
        echo "   Or add $SCRIPT_DIR/formatter to your PATH"
    fi
}

# Function to test installation
test_installation() {
    echo "🧪 Testing installation..."
    
    # Test formatter if available
    if command -v flowfmt &> /dev/null; then
        echo "✅ Formatter is available globally as 'flowfmt'"
    elif [[ -x "$SCRIPT_DIR/formatter/flowfmt.py" ]]; then
        echo "✅ Formatter is available at $SCRIPT_DIR/formatter/flowfmt.py"
    else
        echo "❌ Formatter installation failed"
    fi
    
    echo "✅ Installation test completed"
}

# Main installation menu
show_menu() {
    echo "Select editors to install FlowLang support for:"
    echo "1) VS Code"
    echo "2) Sublime Text"
    echo "3) Vim"
    echo "4) Neovim"
    echo "5) Code Formatter"
    echo "6) All of the above"
    echo "7) Test installation"
    echo "8) Exit"
    echo
    read -p "Enter your choice (1-8): " choice
}

# Handle command line arguments
if [ $# -gt 0 ]; then
    case $1 in
        "vscode")
            install_vscode
            echo "🎉 VS Code support installed!"
            exit 0
            ;;
        "sublime")
            install_sublime
            echo "🎉 Sublime Text support installed!"
            exit 0
            ;;
        "vim")
            install_vim
            echo "🎉 Vim support installed!"
            exit 0
            ;;
        "neovim")
            install_neovim
            echo "🎉 Neovim support installed!"
            exit 0
            ;;
        "formatter")
            install_formatter
            echo "🎉 Formatter installed!"
            exit 0
            ;;
        "all")
            install_vscode
            install_sublime
            install_vim
            install_neovim
            install_formatter
            echo "🎉 All FlowLang editor support installed!"
            exit 0
            ;;
        "test")
            test_installation
            exit 0
            ;;
        *)
            echo "❌ Unknown option: $1"
            echo "Available options: vscode, sublime, vim, neovim, formatter, all, test"
            exit 1
            ;;
    esac
fi

# Main installation loop (interactive mode)
while true; do
    show_menu
    
    case $choice in
        1)
            install_vscode
            echo
            ;;
        2)
            install_sublime
            echo
            ;;
        3)
            install_vim
            echo
            ;;
        4)
            install_neovim
            echo
            ;;
        5)
            install_formatter
            echo
            ;;
        6)
            install_vscode
            install_sublime
            install_vim
            install_neovim
            install_formatter
            echo
            echo "🎉 All FlowLang editor support installed!"
            echo
            ;;
        7)
            test_installation
            echo
            ;;
        8)
            echo "👋 Installation complete! Happy coding with FlowLang!"
            exit 0
            ;;
        *)
            echo "❌ Invalid option. Please try again."
            echo
            ;;
    esac
done