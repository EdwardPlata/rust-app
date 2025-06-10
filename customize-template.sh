#!/bin/bash
# Template customization helper script
# Run this after creating a repository from the template

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}🦀 Rust Template Customization${NC}"
echo -e "${BLUE}This script will help you customize your new Rust project${NC}"
echo

# Get project name (default to current directory name)
DEFAULT_NAME=$(basename "$(pwd)")
echo -e "${BLUE}Project name${NC} [${YELLOW}${DEFAULT_NAME}${NC}]:"
read -r PROJECT_NAME
PROJECT_NAME=${PROJECT_NAME:-$DEFAULT_NAME}

# Get project type
echo
echo -e "${BLUE}Choose your project type:${NC}"
echo "1) Single binary application"
echo "2) Library crate"
echo "3) Web server/API"
echo "4) CLI tool"
echo "5) WebAssembly project"
echo "6) Keep all examples (workspace)"

while true; do
    echo -e "${BLUE}Select option${NC} [${YELLOW}1${NC}]:"
    read -r CHOICE
    CHOICE=${CHOICE:-1}

    case $CHOICE in
        1|2|3|4|5|6) break;;
        *) echo "Please select 1-6";;
    esac
done

echo
echo -e "${GREEN}Customizing project...${NC}"

# Update Cargo.toml project name
sed -i.bak "s/rust-codespaces/$PROJECT_NAME/g" Cargo.toml
rm -f Cargo.toml.bak

# Customize based on project type
case $CHOICE in
    1) # Binary application
        echo "Setting up binary application..."
        rm -rf examples/web_server examples/cli_tool examples/wasm_example examples/my_lib
        mv examples/hello_world .
        rm -rf examples/
        echo 'members = ["."]' >> Cargo.toml
        ;;
    2) # Library
        echo "Setting up library crate..."
        rm -rf examples/hello_world examples/web_server examples/cli_tool examples/wasm_example
        mv examples/my_lib/* .
        rm -rf examples/
        sed -i.bak 's/\[\[bin\]\]//g' Cargo.toml && rm -f Cargo.toml.bak
        ;;
    3) # Web server
        echo "Setting up web server..."
        rm -rf examples/hello_world examples/cli_tool examples/wasm_example examples/my_lib
        mv examples/web_server .
        rm -rf examples/
        cargo add axum tower serde_json tokio
        ;;
    4) # CLI tool
        echo "Setting up CLI tool..."
        rm -rf examples/hello_world examples/web_server examples/wasm_example examples/my_lib
        mv examples/cli_tool .
        rm -rf examples/
        cargo add clap serde_yaml
        ;;
    5) # WASM
        echo "Setting up WebAssembly project..."
        rm -rf examples/hello_world examples/web_server examples/cli_tool examples/my_lib
        mv examples/wasm_example .
        rm -rf examples/
        rustup target add wasm32-unknown-unknown
        cargo add wasm-bindgen js-sys web-sys
        ;;
    6) # Keep workspace
        echo "Keeping workspace structure..."
        ;;
esac

# Clean up template files
echo "Cleaning up template files..."
rm -f STATUS.md TEMPLATE_USAGE.md README-TEMPLATE.md

# Update README
if [ -f README-TEMPLATE.md ]; then
    mv README-TEMPLATE.md README.md
fi

echo
echo -e "${GREEN}✅ Template customization complete!${NC}"
echo -e "${BLUE}Next steps:${NC}"
echo "1. Update README.md with your project description"
echo "2. Modify Cargo.toml metadata (author, description, etc.)"
echo "3. Test your setup: cargo build && cargo test"
echo "4. Remove this script: rm customize-template.sh"
echo
echo -e "${GREEN}🎉 Happy coding with Rust!${NC}"
