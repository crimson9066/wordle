#!/bin/bash
# Quick reference script for common development tasks

# Initialize the project
init() {
    echo "Installing dependencies..."
    npm install
    echo "Dependencies installed"
}

# Run development server
dev() {
    echo "Starting development server..."
    npm run tauri-dev
}

# Run tests
test() {
    echo "Running tests..."
    npm test
}

# Build for production
build() {
    echo "Building for production..."
    npm run tauri-build
}

# Run linters
lint() {
    echo "Running linters..."
    npm run lint
    cd src-tauri && cargo clippy && cd ..
}

# Run all checks
check() {
    echo "Running all checks..."
    npm run type-check
    npm run lint
    npm test
    cd src-tauri && cargo test && cd ..
}

# Clean build artifacts
clean() {
    echo "Cleaning build artifacts..."
    rm -rf dist/
    rm -rf src-tauri/target/
    rm -rf node_modules/
}

# Show help
help() {
    echo "Wordle Suite - Development Commands"
    echo ""
    echo "Usage: ./scripts/dev.sh <command>"
    echo ""
    echo "Commands:"
    echo "  init    - Install dependencies"
    echo "  dev     - Start development server"
    echo "  test    - Run tests"
    echo "  build   - Build for production"
    echo "  lint    - Run linters"
    echo "  check   - Run all checks"
    echo "  clean   - Clean build artifacts"
    echo "  help    - Show this help message"
}

# Main
case "${1:-help}" in
    init)  init ;;
    dev)   dev ;;
    test)  test ;;
    build) build ;;
    lint)  lint ;;
    check) check ;;
    clean) clean ;;
    *)     help ;;
esac
