#!/bin/bash
set -e

echo "🔨 Building WASM library with wasm-pack..."
wasm-pack build --target web --out-dir pkg

echo "✅ WASM library built successfully!"
echo ""
echo "Package ready in ./pkg/"
echo ""
echo "Next steps:"
echo "  cd example"
echo "  npm install"
echo "  npm run dev"
echo ""
echo "To publish to npm:"
echo "  cd pkg"
echo "  npm publish"
