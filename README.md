# TachyonFX Renderer

WebAssembly library for rendering [tachyonfx](https://github.com/junkdog/tachyonfx) effects in the browser. Replaces
animated GIFs in documentation with live, interactive terminal effect demos.

**[🚀 Live Demo](https://junkdog.github.io/tachyonfx-renderer/)**

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

// Update effect and replay
renderer.updateEffect('fx::fade_in((600, Interpolation::CubicOut))');
renderer.playEffect();

// Cleanup
renderer.destroy();
```

## API

### `createRenderer(containerId, dslCode, ansiContent)`

Creates a renderer instance.

- `containerId`: DOM element ID for the canvas
- `dslCode`: Effect DSL (see [tachyonfx DSL docs][tfx-dsl] and [fx docs][fx-docs])
- `ansiContent`: ANSI-formatted text

 [tfx-dsl]: https://github.com/junkdog/tachyonfx/blob/development/docs/dsl.md
 [fx-docs]: https://docs.rs/tachyonfx/latest/tachyonfx/fx/index.html

Returns a `TachyonFxRenderer` handle.

### `TachyonFxRenderer` methods

- `updateEffect(dslCode)` - Change and apply a new effect
- `playEffect()` - Replay the current effect
- `destroy()` - Stop rendering and cleanup resources

## Effect Examples

```typescript
// Slide in from right to left
'fx::slide_in(Motion::RightToLeft, 10, 0, Color::Black, (800, Interpolation::QuadOut))'

// Fade in from color
'fx::fade_from_fg(Color::Black, (600, Interpolation::CubicOut))'

// Parallel effects
`fx::parallel(&[
  fx::sweep_in(Motion::RightToLeft, 15, 0, Color::Black, (1000, Interpolation::BounceOut)),
  fx::coalesce((1000, Interpolation::QuadOut))
])`

// Sequential effects
`fx::sequence(&[
  fx::fade_to_fg(Color::Black, (300, Interpolation::Linear)),
  fx::sleep(200),
  fx::fade_from_fg(Color::Black, (300, Interpolation::Linear))
])`
```

## Multiple Instances

Multiple renderers can run independently on the same page:

```typescript
const renderer1 = createRenderer('terminal-1', dsl1, content1);
const renderer2 = createRenderer('terminal-2', dsl2, content2);

renderer1.updateEffect(newDsl);
renderer2.playEffect();
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
