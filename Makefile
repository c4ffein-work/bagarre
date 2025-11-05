.PHONY: help test lint fmt-check fmt clippy build-wasm verify clean

# Default target
help:
	@echo "Bagarre Fighting Game Engine - Development Commands"
	@echo ""
	@echo "Available targets:"
	@echo "  make verify       - Run all checks (format, lint, tests) - REQUIRED before completion"
	@echo "  make test         - Run all tests"
	@echo "  make lint         - Run clippy linter"
	@echo "  make fmt-check    - Check code formatting"
	@echo "  make fmt          - Format code"
	@echo "  make build-wasm   - Build WASM binary"
	@echo "  make clean        - Clean build artifacts"
	@echo ""

# Run all verification checks (matches CI/CD pipeline)
verify:
	@echo "🔍 Running all verification checks..."
	@echo ""
	@$(MAKE) fmt-check
	@echo ""
	@$(MAKE) lint
	@echo ""
	@$(MAKE) test
	@echo ""
	@echo "✅ All verification checks passed!"

# Check code formatting
fmt-check:
	@echo "🎨 Checking code formatting..."
	@cargo fmt --check
	@echo "✅ Code formatting is correct"

# Format code
fmt:
	@echo "🎨 Formatting code..."
	@cargo fmt
	@echo "✅ Code formatted"

# Run clippy linter
lint:
	@echo "📎 Running Clippy linter..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "✅ Clippy checks passed"

# Run all tests
test:
	@echo "🧪 Running test suite..."
	@cargo test --verbose
	@echo "✅ All tests passed"

# Build WASM binary
build-wasm:
	@echo "🦀 Building Bagarre for WebAssembly..."
	@if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then \
		echo "📦 Installing wasm32-unknown-unknown target..."; \
		rustup target add wasm32-unknown-unknown; \
	fi
	@cargo build --target wasm32-unknown-unknown --release
	@echo "📦 Copying WASM to browser demo..."
	@mkdir -p examples/browser
	@cp target/wasm32-unknown-unknown/release/bagarre.wasm examples/browser/
	@echo "✅ WASM build complete: examples/browser/bagarre.wasm"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@echo "✅ Clean complete"
