# TachyonFX Renderer

WebAssembly library for rendering [tachyonfx](https://github.com/junkdog/tachyonfx) effects in the browser. Replaces
animated GIFs in documentation with live, interactive terminal effect demos.

## Installation

```bash
npm install tachyonfx-renderer
```

## Usage

```typescript
import init, { createRenderer } from 'tachyonfx-renderer';

await init();

const renderer = createRenderer(
  'container-id',
  'fx::slide_in(Motion::RightToLeft, 10, 0, Color::Black, (800, Interpolation::QuadOut))',
  '\x1b[32mHello, Terminal Effects!\x1b[0m'
);

// Control playback
renderer.stop();
renderer.start();
renderer.replayEffect();

// Update content or effect
renderer.updateCanvas('\x1b[31mNew content\x1b[0m');
renderer.updateEffect('fx::fade_in((600, Interpolation::CubicOut))');

// Cleanup
renderer.destroy();
```

## API

### `createRenderer(containerId, dslCode, ansiContent)`

Creates a renderer instance.

- `containerId`: DOM element ID for the canvas
- `dslCode`: Effect DSL (see [tachyonfx DSL docs](https://github.com/junkdog/tachyonfx/blob/development/docs/dsl.md))
- `ansiContent`: ANSI-formatted text

Returns a `TachyonRenderer` handle.

### `TachyonRenderer` methods

- `updateCanvas(ansiContent)` - Update displayed content
- `updateEffect(dslCode)` - Change effect
- `replayEffect()` - Restart current effect
- `start()` - Resume rendering
- `stop()` - Pause rendering
- `isRunning()` - Check if active
- `destroy()` - Cleanup and remove

## Effect Examples

```typescript
// Slide in
'fx::slide_in(Motion::LeftToRight, 8, 0, Color::Black, (800, Interpolation::QuadOut))'

// Fade in
'fx::fade_from_fg(Color::Black, (600, Interpolation::CubicOut))'

// Parallel effects
`fx::parallel(&[
  fx::sweep_in(Motion::RightToLeft, 15, 0, Color::Black, (1000, Interpolation::BounceOut)),
  fx::coalesce((1000, Interpolation::QuadOut))
])`
```

## Multiple Instances

Multiple renderers can run independently on the same page:

```typescript
const renderer1 = createRenderer('terminal-1', dsl1, content1);
const renderer2 = createRenderer('terminal-2', dsl2, content2);

renderer1.stop();
renderer2.replayEffect();
```

## Example

See the `example/` directory for a complete TypeScript/webpack setup with dual canvas instances.

```bash
./build-wasm.sh
cd example
npm install
npm run dev
```

## Building from Source

```bash
# Build WASM library
./build-wasm.sh

# Or with wasm-pack directly
wasm-pack build --target web --out-dir pkg
```

## License

MIT
