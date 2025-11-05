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

    # Copy to browser demo directory
    echo "📋 Copying to browser demo..."
    mkdir -p examples/browser
    cp "$WASM_FILE" examples/browser/bagarre.wasm

    echo ""
    echo "✅ WASM module ready at examples/browser/bagarre.wasm"
    echo ""
    echo "To test locally:"
    echo "  cd examples/browser && python3 -m http.server 8000"
    echo "  Then open http://localhost:8000"
else
    echo "❌ Build failed!"
    exit 1
fi
