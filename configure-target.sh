# Configuration for aws-lc-sys
# This file helps resolve common issues with aws-lc-sys compilation

# Environment variables for cross-compilation
export AWS_LC_SYS_CMAKE_BUILDER=1
export AWS_LC_SYS_NO_ASM=1

# To avoid issues with Perl (required by aws-lc-sys)
export AWS_LC_SYS_EXTERNAL_BINDGEN=1

# Configuration for different targets
case "$1" in
    "x86_64-pc-windows-gnu")
        echo "Configuring for Windows x86_64..."
        export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
        export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++
        export AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar
        export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
        ;;
    "x86_64-unknown-linux-gnu")
        echo "Configuring for Linux x86_64..."
        export CC_x86_64_unknown_linux_gnu=gcc
        export CXX_x86_64_unknown_linux_gnu=g++
        ;;
    "x86_64-apple-darwin")
        echo "Configuring for macOS Intel..."
        # Requires macOS-specific tools
        if ! command -v x86_64-apple-darwin-gcc >/dev/null 2>&1; then
            echo "⚠️  macOS cross-compilation tools not available"
            echo "   Skipping macOS Intel compilation"
            exit 0
        fi
        ;;
    "aarch64-apple-darwin")
        echo "Configuring for macOS ARM64..."
        # Requires macOS-specific tools
        if ! command -v aarch64-apple-darwin-gcc >/dev/null 2>&1; then
            echo "⚠️  macOS ARM cross-compilation tools not available"
            echo "   Skipping macOS ARM64 compilation"
            exit 0
        fi
        ;;
    *)
        echo "Target not specified or not supported: $1"
        ;;
esac
