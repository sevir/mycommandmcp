# 🚀 Release Guide with GoReleaser

This guide explains how to use the release system configured for MyCommandMCP.

## 📋 Prerequisites

### 1. Install GoReleaser

#### Linux (Ubuntu/Debian)
```bash
echo 'deb [trusted=yes] https://repo.goreleaser.com/apt/ /' | sudo tee /etc/apt/sources.list.d/goreleaser.list
sudo apt update
sudo apt install goreleaser
```

#### macOS
```bash
brew install goreleaser
```

#### Other options
See: https://goreleaser.com/install/

### 2. Configure cross-compilation
```bash
# Run the configuration script
chmod +x setup-cross-compilation.sh
./setup-cross-compilation.sh
```

Or use the Makefile:
```bash
make setup-cross
```

## 🔧 Configuration

### Environment variables for GitHub
For automatic releases you need to configure:

```bash
export GITHUB_TOKEN="your_github_token"
export GITHUB_REPOSITORY_OWNER="your_username"
export GITHUB_REPOSITORY_NAME="repo_name"
```

The token must have permissions for:
- `repo` (full repository access)
- `write:packages` (if using GitHub Packages)

## 🚀 Create Releases

### 1. Local Release (Testing)

To test without uploading anything to GitHub:

```bash
# Using the script
./release-local.sh

# Or using the Makefile
make release-local

# Or directly with GoReleaser
goreleaser release --snapshot --clean
```

This will create files in the `dist/` folder for all platforms.

### 2. GitHub Release

#### Method 1: Manual

```bash
# 1. Update version in Cargo.toml if necessary
# 2. Commit all changes
git add .
git commit -m "chore: prepare release v1.0.0"

# 3. Create tag
git tag v1.0.0

# 4. Push the tag
git push origin v1.0.0

# 5. Create release
make release
# Or directly: goreleaser release --clean
```

#### Method 2: GitHub Actions (Automatic)

The workflow is configured to run automatically when:
- A tag starting with `v*` is pushed
- Manually executed from GitHub Actions

```bash
# You only need to create and push the tag
git tag v1.0.0
git push origin v1.0.0
```

## 📦 Generated Files

Each release includes:

### Binaries by platform:
- `mycommandmcp_v1.0.0_linux_amd64.tar.gz`
- `mycommandmcp_v1.0.0_windows_amd64.zip`
- `mycommandmcp_v1.0.0_darwin_amd64.tar.gz` (Intel Mac)
- `mycommandmcp_v1.0.0_darwin_arm64.tar.gz` (Apple Silicon)

### Files included in each package:
- Executable binary (`mycommandmcp` or `mycommandmcp.exe`)
- `README.md`
- `LICENSE.txt`
- `USAGE.md`
- `mycommand-tools.yaml`
- `mycommand-tools-extended.yaml`

### Additional files:
- `checksums.txt` - SHA256 checksums of all files

## 🛠️ Useful Commands

### Makefile
```bash
# View all available commands
make help

# Compile for all platforms
make build-all

# Run tests
make test

# Clean compilation files
make clean

# Check current version
make check-version

# Local release for testing
make release-local
```

### Direct Cargo commands
```bash
# Compile for Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Compile for Windows
cargo build --release --target x86_64-pc-windows-gnu

# Compile for macOS Intel
cargo build --release --target x86_64-apple-darwin

# Compile for macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin
```

## 🔍 Verification

### Verify installed targets
```bash
rustup target list --installed
```

### Verify cross-compilation
```bash
# Verify that it can compile for all targets
cargo check --target x86_64-unknown-linux-gnu
cargo check --target x86_64-pc-windows-gnu
cargo check --target x86_64-apple-darwin
cargo check --target aarch64-apple-darwin
```

### Verify GoReleaser
```bash
# Verify configuration
goreleaser check

# See what a release would do without executing it
goreleaser release --snapshot --skip-publish --clean
```

## 🐛 Troubleshooting

### Error: linker `x86_64-w64-mingw32-gcc` not found
```bash
# Ubuntu/Debian
sudo apt-get install gcc-mingw-w64-x86-64

# macOS
brew install mingw-w64
```

### Error: target not installed
```bash
# Install missing target
rustup target add x86_64-pc-windows-gnu
```

### GitHub permissions error
- Verify that the token has the correct permissions
- Verify that environment variables are configured

### Compilation fails for macOS on Linux
To compile for macOS from Linux you need additional tools:
```bash
# Install osxcross (advanced)
# Or use GitHub Actions which already has the tools
```

## 📝 Customization

### Modify platforms
Edit `.goreleaser.yml` to add/remove platforms in the `before.hooks` section.

### Modify included files
Edit the `archives` section in `.goreleaser.yml`.

### Modify changelog format
Edit the `changelog` section in `.goreleaser.yml`.

## 🔗 Useful Links

- [GoReleaser Documentation](https://goreleaser.com/)
- [Rust Cross Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)
- [GitHub Actions for Rust](https://docs.github.com/en/actions/automating-builds-and-tests/building-and-testing-rust)
