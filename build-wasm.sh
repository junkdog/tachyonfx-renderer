#!/bin/bash
set -e

echo "🔨 Building WASM library..."
cargo build --target wasm32-unknown-unknown --release

echo "📦 Generating TypeScript bindings..."
wasm-bindgen \
  --target web \
  --typescript \
  --out-dir ./pkg \
  target/wasm32-unknown-unknown/release/tachyonfx_renderer.wasm

echo "✅ WASM library built successfully!"
echo ""
echo "Next steps:"
echo "  cd example"
echo "  npm install"
echo "  npm run dev"
