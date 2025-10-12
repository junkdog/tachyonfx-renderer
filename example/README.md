# TachyonFX Renderer - TypeScript/Webpack Example

This example demonstrates how to use the TachyonFX Renderer WebAssembly library in a TypeScript application with webpack.

## Quick Start

**1. Build the WASM library** (from project root):

```bash
./build-wasm.sh
```

Or manually:
```bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --typescript --out-dir ./pkg target/wasm32-unknown-unknown/release/tachyonfx_renderer.wasm
```

**2. Install dependencies and run**:

```bash
cd example
npm install
npm run dev
```

Then open http://localhost:8080

## Running the Example

### Development Mode (with hot reload)

```bash
npm run dev
```

Then open http://localhost:8080 in your browser.

## API Usage

The example demonstrates all methods from the TachyonFX Renderer API:

### Creating a Renderer

```typescript
import init, { createRenderer } from './pkg/tachyonfx_renderer.js';

await init();

const renderer = createRenderer(
  'my-canvas-id',          // Container element ID
  'fx: [slide_in(...)]',   // Effect DSL
  ansiContent              // ANSI-formatted content
);
```

### Updating Content

```typescript
// Update the ANSI content
renderer.updateCanvas('\x1b[32mNew content\x1b[0m');

// Change the effect
renderer.updateEffect('fx: [fade_in(600ms)]');

// Replay current effect
renderer.replayEffect();
```

### Lifecycle Control

```typescript
// Pause rendering
renderer.stop();

// Resume rendering
renderer.start();

// Check if running
const isRunning = renderer.isRunning(); // boolean

// Clean up
renderer.destroy();
```

## Effect DSL Examples

The example includes several effect DSL snippets:

```typescript
// Simple slide-in
const effect = 'fx: [slide_in(direction: right, 800ms)]';

// Sweep with easing
const effect = 'fx: [sweep_in(origin: top_left, 1200ms, quad_out)]';

// Complex parallel effects
const effect = `
fx: [
  parallel: [
    sweep_in(origin: center, 1000ms, bounce_out),
    coalesce(fg, 1000ms, quad_out, start_at: 20%),
  ]
]
`;
```
