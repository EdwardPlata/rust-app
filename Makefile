# Makefile for Rust development workflows

.PHONY: help build test clean fmt clippy run install-tools bench doc check-ci setup-hooks

# Default target
help:
	@echo "🦀 Rust Development Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  build        - Build all workspace members"
	@echo "  test         - Run tests for all workspace members"
	@echo "  test-watch   - Run tests in watch mode"
	@echo "  fmt          - Format all Rust code"
	@echo "  clippy       - Run clippy linter"
	@echo "  clean        - Clean build artifacts"
	@echo "  run-hello    - Run the hello_world example"
	@echo "  run-web      - Run the web server example"
	@echo "  run-cli      - Run the CLI tool example"
	@echo "  bench        - Run benchmarks"
	@echo "  doc          - Generate documentation"
	@echo "  wasm-build   - Build WASM example"
	@echo "  wasm-serve   - Serve WASM example locally"
	@echo "  install-tools - Install additional development tools"
	@echo "  audit        - Security audit of dependencies"
	@echo "  outdated     - Check for outdated dependencies"
	@echo "  check-ci     - Run CI checks locally"
	@echo "  setup-hooks  - Install Git pre-commit hooks"

# Build all workspace members
build:
	cargo build --all

# Build in release mode
build-release:
	cargo build --all --release

# Run tests
test:
	cargo test --all

# Run tests in watch mode
test-watch:
	cargo watch -x "test --all"

# Format code
fmt:
	cargo fmt --all

# Run clippy
clippy:
	cargo clippy --all -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Run examples
run-hello:
	cargo run -p hello_world

run-web:
	cargo run -p web_server

run-cli:
	@echo "Available CLI commands:"
	@echo "  make run-cli-greet NAME=<name>"
	@echo "  make run-cli-factorial NUM=<number>"
	@echo "  make run-cli-file-create PATH=<path>"

run-cli-greet:
	cargo run -p cli_tool -- greet --name "${NAME}"

run-cli-factorial:
	cargo run -p cli_tool -- factorial ${NUM}

run-cli-file-create:
	cargo run -p cli_tool -- file create ${PATH}

# Run benchmarks
bench:
	cargo bench

# Generate documentation
doc:
	cargo doc --all --open

# WASM development
wasm-build:
	cd examples/wasm_example && wasm-pack build --target web

wasm-serve: wasm-build
	cd examples/wasm_example && python3 -m http.server 8000

# Development tools
install-tools:
	cargo install cargo-watch cargo-edit cargo-tree cargo-audit cargo-outdated
	cargo install wasm-pack trunk
	cargo install flamegraph hyperfine

# Security and maintenance
audit:
	cargo audit

outdated:
	cargo outdated

# Update dependencies
update:
	cargo update

# Run all checks (useful for CI/pre-commit)
check-all: fmt clippy test
	@echo "✅ All checks passed!"

# Development workflow
dev: fmt clippy test
	@echo "🚀 Ready for development!"

# CI testing
check-ci:
	@echo "🔧 Running CI checks locally..."
	./test_ci.sh

# Setup Git hooks
setup-hooks:
	@echo "🔧 Setting up Git hooks..."
	./setup-hooks.sh
