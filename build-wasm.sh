#!/bin/bash
# Build script for WASM compilation

set -e

echo "🥊 Bagarre - Building for WASM..."

# Check if wasm32 target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "📦 Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Build for WASM
echo "🔨 Compiling to WASM..."
cargo build --target wasm32-unknown-unknown --release

# Output location
WASM_FILE="target/wasm32-unknown-unknown/release/bagarre.wasm"

if [ -f "$WASM_FILE" ]; then
    SIZE=$(du -h "$WASM_FILE" | cut -f1)
    echo "✅ Build successful!"
    echo "📦 WASM file: $WASM_FILE ($SIZE)"
    echo ""
    echo "To use in browser:"
    echo "  1. Copy $WASM_FILE to your web directory"
    echo "  2. Load it with WebAssembly.instantiateStreaming()"
    echo "  3. See examples/browser/ for a demo"
    echo ""
    echo "Optional: Use wasm-bindgen for easier JS interop"
    echo "  cargo install wasm-bindgen-cli"
    echo "  wasm-bindgen $WASM_FILE --out-dir examples/browser --target web"
else
    echo "❌ Build failed!"
    exit 1
fi
