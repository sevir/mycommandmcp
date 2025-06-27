#!/bin/bash

# Script to create a local release using GoReleaser
# Useful for testing before making the real release

set -e

echo "🚀 Creating local release with GoReleaser..."

# Verify that GoReleaser is installed
if ! command -v goreleaser &> /dev/null; then
    echo "❌ GoReleaser is not installed"
    echo "📦 Installing GoReleaser..."
    
    # Detect the operating system and install GoReleaser
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        echo "deb [trusted=yes] https://repo.goreleaser.com/apt/ /" | sudo tee /etc/apt/sources.list.d/goreleaser.list
        sudo apt update
        sudo apt install goreleaser
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        brew install goreleaser
    else
        echo "❌ Operating system not supported for automatic installation"
        echo "Please install GoReleaser manually: https://goreleaser.com/install/"
        exit 1
    fi
fi

# Verify that Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo is not installed"
    echo "Please install Rust: https://rustup.rs/"
    exit 1
fi

# Prepare cross-compilation environment
echo "🔧 Preparing cross-compilation environment..."
if [ -f "./setup-cross-compilation.sh" ]; then
    chmod +x ./setup-cross-compilation.sh
    ./setup-cross-compilation.sh
else
    echo "⚠️  setup-cross-compilation.sh file not found"
    echo "📦 Setting up basic targets..."
    rustup target add x86_64-unknown-linux-gnu
    rustup target add x86_64-pc-windows-gnu
    rustup target add x86_64-apple-darwin
    rustup target add aarch64-apple-darwin
fi

# Create a snapshot release (not uploaded to GitHub)
echo "📦 Creating snapshot release..."
goreleaser release --snapshot --clean

echo "✅ Local release created successfully!"
echo ""
echo "📋 Files generated in:"
echo "  📁 dist/"
echo ""
echo "🔍 To see generated files:"
echo "  ls -la dist/"
echo ""
echo "🚀 To create a real GitHub release:"
echo "  1. Commit all changes"
echo "  2. Create a tag: git tag v1.0.0"
echo "  3. Push the tag: git push origin v1.0.0"
echo "  4. Run: goreleaser release --clean"
echo ""
echo "💡 Or use GitHub Actions to automate the process"
