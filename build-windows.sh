#!/bin/bash

# Specific script to compile for Windows with aws-lc-sys
# This script configures all necessary environment variables

set -e

echo "🪟 Compiling for Windows x86_64..."

# Configure environment variables for aws-lc-sys
# export AWS_LC_SYS_NO_ASM=1  # Only for debug builds
export AWS_LC_SYS_CMAKE_BUILDER=1
export CMAKE=/usr/bin/cmake
export CMAKE_x86_64_pc_windows_gnu=/usr/bin/cmake

# Configure cross compilers
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++
export AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

# Additional variables for aws-lc-sys specific to the target
export AWS_LC_SYS_CMAKE_x86_64_pc_windows_gnu=/usr/bin/cmake
# export AWS_LC_SYS_NO_ASM_x86_64_pc_windows_gnu=1  # Only for debug

# Verify that tools are available
echo "🔍 Verifying tools..."
if ! command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then
    echo "❌ x86_64-w64-mingw32-gcc not found"
    echo "Install mingw-w64: sudo apt-get install gcc-mingw-w64-x86-64"
    exit 1
fi

if ! command -v cmake >/dev/null 2>&1; then
    echo "❌ cmake not found"
    echo "Install cmake: sudo apt-get install cmake"
    exit 1
fi

echo "✅ Tools verified"
echo "🔨 Starting compilation..."

# Clean previous cache if necessary
cargo clean --target x86_64-pc-windows-gnu

# Compile
cargo build --release --target x86_64-pc-windows-gnu

if [ $? -eq 0 ]; then
    echo "✅ Windows compilation completed successfully"
    echo "📁 Binary available at: target/x86_64-pc-windows-gnu/release/mycommandmcp.exe"
else
    echo "❌ Error in Windows compilation"
    echo "💡 Try using Linux-only compilation:"
    echo "   cargo build --release --target x86_64-unknown-linux-gnu"
    exit 1
fi
