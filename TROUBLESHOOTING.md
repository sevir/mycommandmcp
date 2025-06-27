# 🔧 Troubleshooting - MyCommandMCP Release

## aws-lc-sys Error in Cross Compilation

### Problem
```
error: failed to run custom build command for `aws-lc-sys v0.29.0`
Missing dependency: cmake
```

### ✅ Implemented Solutions

#### 1. Specialized Script for Windows
We've created `build-windows.sh` that configures all necessary environment variables:

```bash
# Use the specialized script
./build-windows.sh
```

#### 2. Configuration in .cargo/config.toml
Pre-configured environment variables for aws-lc-sys:

```toml
[env]
AWS_LC_SYS_CMAKE_BUILDER = "1"
CMAKE = "/usr/bin/cmake"
CMAKE_x86_64_pc_windows_gnu = "/usr/bin/cmake"
```

#### 3. Makefile with Flexible Options
```bash
# Safe compilation (only available platforms)
make build-safe

# Windows only (if tools are available)
make build-windows

# All platforms (will attempt all)
make build-all
```

### 🚀 Automatic Releases with GitHub Actions

To avoid local cross-compilation issues, use GitHub Actions:

1. **Push a tag:**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **GitHub Actions automatically:**
   - Sets up the correct environment
   - Compiles for all platforms
   - Creates the release on GitHub

### 💡 Workarounds by Platform

#### Linux ✅
- **Native:** `cargo build --release --target x86_64-unknown-linux-gnu`
- **Windows:** `./build-windows.sh`
- **macOS:** Only on GitHub Actions (requires special tools)

#### Windows ✅
- **Native:** `cargo build --release`
- **Linux:** With WSL + cross-compilation tools
- **macOS:** Only on GitHub Actions

#### macOS ✅
- **Native:** `cargo build --release`
- **Windows:** With Homebrew + mingw-w64
- **Linux:** Native cross-compilation

### 🏗️ Environment Setup

#### Ubuntu/Debian
```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y \
    gcc-mingw-w64-x86-64 \
    build-essential \
    cmake \
    git \
    pkg-config \
    libssl-dev \
    zlib1g-dev

# Configure targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

#### macOS
```bash
# Install dependencies
brew install mingw-w64 cmake git pkg-config

# Configure targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
```

### 🔍 Environment Verification

```bash
# Verify tools
make check-version

# Setup cross-compilation
make setup-cross

# Test local release
make release-local
```

### 📦 Release Structure

Each release includes:
- `mycommandmcp_v1.0.0_linux_amd64.tar.gz`
- `mycommandmcp_v1.0.0_windows_amd64.zip`
- `mycommandmcp_v1.0.0_darwin_amd64.tar.gz` (Intel Mac)
- `mycommandmcp_v1.0.0_darwin_arm64.tar.gz` (Apple Silicon)
- `checksums.txt`

### 🐛 Known Issues

1. **aws-lc-sys compilation:** 
   - Requires target-specific cmake
   - Some flags only work on debug builds
   - **Solution:** Platform-specialized scripts

2. **macOS cross-compilation on Linux:**
   - Requires osxcross or special tools
   - **Solution:** Use GitHub Actions for macOS

3. **Environment variables:**
   - aws-lc-sys is very sensitive to configuration
   - **Solution:** Pre-established configuration in `.cargo/config.toml`

### 📞 Support

If you encounter issues:

1. **Check dependencies:** `make check-version`
2. **Use safe compilation:** `make build-safe`
3. **For releases:** Use GitHub Actions with tags
4. **For local development:** `cargo build --release` (native platform)

### 🎯 Recommendations

- **Development:** Native compilation + local testing
- **CI/CD:** GitHub Actions for multi-platform releases
- **Testing:** `make release-local` to verify configuration
- **Production:** Tags + GitHub Actions for official releases
