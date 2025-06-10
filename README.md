# 🦀 Rust Development Template

A production-ready Rust development template with **GitHub Codespaces** support, complete examples, and modern tooling.

## 🚀 Quick Start

### Option 1: GitHub Codespaces (Recommended)

1. Click **"Use this template"** → **"Create a new repository"**
2. Open your new repository
3. Click **"Code"** → **"Create codespace on main"**
4. Wait for the environment to initialize (2-3 minutes)
5. Start coding! 🎉

### Option 2: Local Development

```bash
git clone <your-repo-url>
cd <your-repo-name>
cargo build
cargo test
```

### 🦀 Rust Toolchain & Components

- **Rust Stable Toolchain**: Latest stable Rust compiler and standard library
- **rustfmt**: Code formatting tool
- **Clippy**: Rust linter for catching common mistakes and improving code
- **rust-analyzer**: Powerful language server for IDE features
- **rust-src**: Rust source code for better IDE integration
- **WASM target**: `wasm32-unknown-unknown` for WebAssembly development

### 🛠️ Development Tools

- **cargo-watch**: Automatically run commands when files change
- **cargo-edit**: Add, remove, and upgrade dependencies from the command line
- **cargo-tree**: Display dependency trees
- **cargo-audit**: Security vulnerability scanner
- **cargo-outdated**: Check for outdated dependencies
- **cargo-expand**: Show macro-expanded code
- **cargo-bloat**: Find what takes most space in your executable
- **cargo-deps**: Build dependency graphs
- **cargo-generate**: Generate projects from templates

### 🌐 Web & WASM Development

- **wasm-pack**: Build Rust-generated WebAssembly
- **trunk**: WASM web application bundler
- **Node.js**: For web development and tooling

### 🗄️ Database Tools

- **sqlx-cli**: SQLx command-line tool for database migrations
- **diesel_cli**: Diesel ORM command-line tool (PostgreSQL & SQLite support)

### 📊 Performance & Debugging

- **tokio-console**: Debugger for async Rust programs
- **flamegraph**: Generate flame graphs for performance analysis
- **hyperfine**: Command-line benchmarking tool
- **LLDB**: Native debugger integration

### 🔧 System Tools

- **ripgrep**: Ultra-fast text search
- **fd-find**: Fast and user-friendly alternative to find
- **bat**: Syntax-highlighted file viewer
- **exa**: Modern replacement for ls
- **starship**: Customizable shell prompt
- **Git**: Version control
- **GitHub CLI**: GitHub integration
- **Docker**: Container support
- **CMake & Clang**: For C/C++ interop

### 🎨 VS Code Extensions

- **rust-analyzer**: Rust language server
- **CodeLLDB**: Native debugger
- **crates**: Manage Rust dependencies
- **Even Better TOML**: Enhanced TOML support
- **Rust Syntax**: Additional syntax highlighting
- **Dependi**: Dependency management
- **GitHub Pull Requests**: GitHub integration
- **Docker**: Container development
- **Live Share**: Collaborative editing
- **IntelliCode**: AI-assisted development

## 🏗️ Project Architecture

This template demonstrates modern Rust project organization with a **workspace structure** and **shared utilities**:

### 📦 Workspace Structure

```
├── shared/              # Shared utilities crate
│   ├── src/
│   │   ├── lib.rs      # Common functions and types
│   │   ├── math.rs     # Mathematical utilities
│   │   ├── text.rs     # Text processing utilities
│   │   └── types.rs    # Shared data structures
│   └── Cargo.toml
├── examples/
│   ├── hello_world/    # Simple binary application
│   ├── cli_tool/       # Command-line tool using clap
│   ├── my_lib/         # Library crate with comprehensive tests
│   ├── wasm_example/   # WebAssembly library
│   └── web_server/     # Axum-based web server
└── Cargo.toml          # Workspace configuration
```

### 🔗 Code Reuse & Best Practices

The template showcases **professional Rust development patterns**:

- **Shared utilities crate**: Common functions (`factorial`, `fibonacci`, `add`, etc.) used across examples
- **Shared types**: `Message` and `UserInfo` structs for consistent data modeling
- **Workspace dependencies**: Centralized dependency management
- **Modular design**: Well-organized code with clear separation of concerns
- **Comprehensive testing**: Unit tests, doc tests, and integration examples

### 📁 Example Projects

- **hello_world**: Basic binary with async operations and structured output
- **cli_tool**: Feature-rich CLI using `clap` with file operations and shared utilities
- **my_lib**: Production-ready library with benchmarks, comprehensive tests, and calculator functionality
- **wasm_example**: WebAssembly library demonstrating browser integration
- **web_server**: RESTful API server using `axum` with JSON endpoints and shared data types

## 🚀 Quick Start

### 1. Create Your Codespace

Click the "Use this template" button or create a new Codespace from this repository.

### 2. Wait for Setup

The environment will automatically install all Rust tools and dependencies. This takes a few minutes on first setup.

### 3. Explore the Examples!

```bash
# Try the improved CLI tool with shared utilities
cd examples/cli_tool
cargo run -- greet --name "Rustacean" --count 3
cargo run -- factorial 10
cargo run -- file create hello.json --content "Hello from Rust!"

# Test the library with benchmarks
cd ../my_lib
cargo test
cargo bench

# Run the web server
cd ../web_server
cargo run
# Visit http://localhost:3000 in another terminal or browser

# Build WebAssembly example
cd ../wasm_example
wasm-pack build --target web
```

### 4. Start Your Own Project

The template demonstrates workspace organization - create your own crate:

```bash
# Add a new binary crate to the workspace
cargo new examples/my_new_project

# Add a new library crate
cargo new shared_utils --lib

# Add to Cargo.toml workspace members
```

## 🔧 Advanced Usage

### WebAssembly Development

```bash
# Create a new WASM project
cargo new --lib my-wasm-project
cd my-wasm-project

# Add WASM dependencies to Cargo.toml
# Then build for WASM
wasm-pack build --target web

# Or use trunk for full web apps
trunk serve
```

### Database Development

```bash
# SQLx setup
cargo install sqlx-cli
sqlx database create
sqlx migrate add initial

# Diesel setup
diesel setup
diesel migration generate create_users
```

### Performance Analysis

```bash
# Benchmark your code
cargo install criterion
# Add criterion to your Cargo.toml, then:
cargo bench

# Profile with flamegraph
cargo flamegraph --bin my-binary

# Benchmark commands
hyperfine 'cargo run --release'
```

## 🎯 Debugging

The environment includes full debugging support:

1. Set breakpoints in VS Code
2. Press F5 or use "Run and Debug"
3. Use the integrated LLDB debugger
4. Debug async code with tokio-console

## 🔍 Code Quality

Built-in code quality tools:

```bash
# Lint your code
cargo clippy

# Format your code
cargo fmt

# Security audit
cargo audit

# Check for outdated dependencies
cargo outdated

# Analyze binary size
cargo bloat --release
```

## 🌐 Port Forwarding

The following ports are pre-configured for forwarding:

- **3000**: Development servers (Trunk, etc.)
- **8080**: Web servers
- **8000**: HTTP servers

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and enhancement requests.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Happy Rust coding! 🦀✨**
