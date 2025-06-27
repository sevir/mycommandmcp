# Makefile for MyCommandMCP
# Facilitates common build, test and release tasks

.PHONY: help build test clean setup-cross release-local release install-deps

# Variables
PROJECT_NAME := mycommandmcp
VERSION := $(shell grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

# Default target
help: ## Shows this help
	@echo "MyCommandMCP v$(VERSION)"
	@echo ""
	@echo "Available commands:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install-deps: ## Installs necessary dependencies
	@echo "📦 Installing Rust dependencies..."
	cargo build
	@echo "✅ Dependencies installed"

build: ## Compiles the project
	@echo "🔨 Compiling project..."
	cargo build --release
	@echo "✅ Compilation completed"

test: ## Runs tests
	@echo "🧪 Running tests..."
	cargo test
	@echo "✅ Tests completed"

clean: ## Cleans compilation files
	@echo "🧹 Cleaning compilation files..."
	cargo clean
	rm -rf dist/
	@echo "✅ Cleanup completed"

setup-cross: ## Configures cross-compilation
	@echo "🔧 Configuring cross-compilation..."
	chmod +x setup-cross-compilation.sh
	./setup-cross-compilation.sh
	@echo "✅ Cross-compilation configured"

release-local: setup-cross ## Creates a local release for testing
	@echo "🚀 Creating local release..."
	chmod +x release-local.sh
	./release-local.sh
	@echo "✅ Local release created"

release: ## Creates a real release (requires tag)
	@echo "🚀 Creating release on GitHub..."
	@if [ -z "$(shell git tag --points-at HEAD)" ]; then \
		echo "❌ No tag on current commit"; \
		echo "💡 Create a tag first: git tag v$(VERSION)"; \
		echo "💡 And then push: git push origin v$(VERSION)"; \
		exit 1; \
	fi
	goreleaser release --clean

check-version: ## Checks current version
	@echo "📋 Project information:"
	@echo "  Name: $(PROJECT_NAME)"
	@echo "  Version: $(VERSION)"
	@echo "  Git tags: $(shell git tag --list | tail -5)"

build-all: setup-cross ## Compiles for all platforms
	@echo "🔨 Compiling for all platforms..."
	@echo "📋 Compiling for Linux x86_64..."
	@source ./configure-target.sh x86_64-unknown-linux-gnu && cargo build --release --target x86_64-unknown-linux-gnu
	@echo "📋 Compiling for Windows x86_64..."
	@./build-windows.sh
	@echo "📋 Compiling for macOS Intel..."
	@source ./configure-target.sh x86_64-apple-darwin && cargo build --release --target x86_64-apple-darwin || echo "⚠️  macOS Intel build skipped"
	@echo "📋 Compiling for macOS ARM64..."
	@source ./configure-target.sh aarch64-apple-darwin && cargo build --release --target aarch64-apple-darwin || echo "⚠️  macOS ARM build skipped"
	@echo "✅ Multi-platform compilation completed"

build-safe: setup-cross ## Compiles only for available platforms
	@echo "🔨 Compiling for available platforms..."
	@echo "📋 Compiling for Linux x86_64..."
	cargo build --release --target x86_64-unknown-linux-gnu
	@if command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then \
		echo "📋 Compiling for Windows x86_64..."; \
		./build-windows.sh; \
	else \
		echo "⚠️  mingw-w64 not available, skipping Windows build"; \
	fi
	@echo "✅ Safe compilation completed"

build-windows: ## Compiles only for Windows
	@echo "🪟 Compiling only for Windows..."
	@./build-windows.sh

install: build ## Installs the binary locally
	@echo "📦 Installing $(PROJECT_NAME)..."
	cargo install --path .
	@echo "✅ $(PROJECT_NAME) installed"

# Development targets
dev: ## Compiles and runs in development mode
	cargo run

watch: ## Automatically compiles when there are changes
	cargo watch -x build

fmt: ## Formats the code
	cargo fmt

lint: ## Runs the linter
	cargo clippy -- -D warnings

# Meta information
info: check-version ## Shows project information
	@echo ""
	@echo "🦀 Rust information:"
	@rustc --version
	@cargo --version
	@echo ""
	@echo "📁 Project structure:"
	@tree -L 2 -I target

# CI/CD targets
ci-test: ## Runs tests for CI
	cargo test --all-features

ci-build: setup-cross build-all ## Build for CI

# Deep cleanup
deep-clean: clean ## Deep cleanup including Cargo cache
	cargo clean
	rm -rf ~/.cargo/registry/index/*
	rm -rf ~/.cargo/registry/cache/*
	rm -rf dist/
