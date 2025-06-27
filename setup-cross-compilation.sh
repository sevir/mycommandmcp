#!/bin/bash

# Script to prepare cross-compilation environment for Rust
# This script should be run before using GoReleaser

set -e

echo "🦀 Preparing Rust cross-compilation environment..."

# Add targets for cross-compilation
echo "📦 Adding Rust targets..."

# Linux x86_64 (already available by default on most systems)
rustup target add x86_64-unknown-linux-gnu

# Windows x86_64
rustup target add x86_64-pc-windows-gnu

# macOS x86_64
rustup target add x86_64-apple-darwin

# macOS ARM64 (Apple Silicon)
rustup target add aarch64-apple-darwin

echo "✅ Targets added successfully"

# Verify that targets are installed
echo "🔍 Installed targets:"
rustup target list --installed

# Install necessary tools for cross-compilation
echo "🔧 Installing cross-compilation tools..."

# Install system dependencies for aws-lc-sys and other dependencies
if command -v apt-get >/dev/null 2>&1; then
    echo "📦 Installing system dependencies (Ubuntu/Debian)..."
    sudo apt-get update
    sudo apt-get install -y \
        gcc-mingw-w64-x86-64 \
        build-essential \
        cmake \
        git \
        pkg-config \
        libssl-dev \
        zlib1g-dev
elif command -v yum >/dev/null 2>&1; then
    echo "📦 Installing system dependencies (CentOS/RHEL)..."
    sudo yum install -y \
        mingw64-gcc \
        gcc \
        gcc-c++ \
        cmake \
        git \
        pkgconfig \
        openssl-devel \
        zlib-devel
elif command -v dnf >/dev/null 2>&1; then
    echo "📦 Installing system dependencies (Fedora)..."
    sudo dnf install -y \
        mingw64-gcc \
        gcc \
        gcc-c++ \
        cmake \
        git \
        pkgconfig \
        openssl-devel \
        zlib-devel
elif command -v brew >/dev/null 2>&1; then
    echo "📦 Installing system dependencies (macOS)..."
    brew install mingw-w64 cmake git pkg-config
else
    echo "⚠️  Unrecognized operating system. Install manually:"
    echo "   - mingw-w64 (for Windows compilation)"
    echo "   - cmake"
    echo "   - git"
    echo "   - pkg-config"
    echo "   - openssl-dev"
fi

# Configure linkers for cross-compilation
echo "🔗 Configuring linkers..."

# Create or update .cargo/config.toml
mkdir -p .cargo
cat > .cargo/config.toml << 'EOF'
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"

[target.x86_64-unknown-linux-gnu]
linker = "cc"

[target.x86_64-apple-darwin]
linker = "cc"

[target.aarch64-apple-darwin]
linker = "cc"
EOF

echo "✅ Linker configuration completed"

# Verify that Rust can compile for all targets
echo "🧪 Verifying compilation for all targets..."

echo "  - Compiling for Linux x86_64..."
cargo check --target x86_64-unknown-linux-gnu

echo "  - Verifying tools for Windows x86_64..."
if command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then
    echo "  ✅ Windows tools available"
    echo "  💡 To compile Windows: use ./build-windows.sh"
else
    echo "  ⚠️  Windows tools not available"
fi

echo "  - Verifying tools for macOS x86_64..."
if command -v x86_64-apple-darwin-gcc >/dev/null 2>&1; then
    echo "  ✅ macOS Intel tools available"
    cargo check --target x86_64-apple-darwin
else
    echo "  ⚠️  macOS Intel tools not available"
    echo "     (This is normal on Linux - use GitHub Actions for macOS)"
fi

echo "  - Verifying tools for macOS ARM64..."
if command -v aarch64-apple-darwin-gcc >/dev/null 2>&1; then
    echo "  ✅ macOS ARM tools available"
    cargo check --target aarch64-apple-darwin
else
    echo "  ⚠️  macOS ARM tools not available"
    echo "     (This is normal on Linux - use GitHub Actions for macOS)"
fi

echo "🎉 Cross-compilation environment configured successfully!"
echo ""
echo "📋 Next steps:"
echo "  1. Make sure you have GoReleaser installed"
echo "  2. Configure GITHUB_TOKEN environment variables"
echo "  3. Run 'goreleaser release' to create a release"
echo "  4. Or run 'goreleaser release --snapshot --clean' for a local test"
