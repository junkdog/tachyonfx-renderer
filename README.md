# TachyonFX Renderer

[![NPM Badge]][NPM]

WebAssembly library for rendering [tachyonfx](https://github.com/junkdog/tachyonfx) effects in the browser. Replaces
animated GIFs in documentation with live, interactive terminal effect demos.

**[🚀 Live Demo](https://junkdog.github.io/tachyonfx-renderer/)**

## Installation

```bash
npm install tachyonfx-renderer
```

## Usage

```typescript
import init, { createRenderer, RendererConfig } from 'tachyonfx-renderer';

await init();

const config = new RendererConfig('container-id')
  .withDsl('fx::slide_in(Motion::RightToLeft, 10, 0, Color::Black, (800, Interpolation::QuadOut))')
  .withCanvas('\x1b[32mHello, Terminal Effects!\x1b[0m')
  .withSleepBetweenReplay(1500);  // Optional: 1.5s between automatic replays

const renderer = createRenderer(config);

// Update effect and restart
renderer.updateEffect('fx::fade_in((600, Interpolation::CubicOut))');
renderer.restartEffect();

// Cleanup
renderer.destroy();
```

## API

### `RendererConfig`

Builder for renderer configuration.

#### Constructor
- `new RendererConfig(containerId)` - Create config with container DOM element ID

#### Builder Methods
- `withDsl(dslCode)` - Set effect DSL (see [tachyonfx DSL docs][tfx-dsl] and [fx docs][fx-docs])
- `withCanvas(ansiContent)` - Set ANSI-formatted text content
- `withSleepBetweenReplay(sleepMs)` - Optional: Set milliseconds between automatic effect replays

 [tfx-dsl]: https://github.com/junkdog/tachyonfx/blob/development/docs/dsl.md
 [fx-docs]: https://docs.rs/tachyonfx/latest/tachyonfx/fx/index.html

### `createRenderer(config)`

Creates a renderer instance from configuration.

Returns a `TachyonFxRenderer` handle.

### `TachyonFxRenderer` methods

- `updateEffect(dslCode)` - Change and apply a new effect
- `restartEffect()` - Restart the current effect from the beginning
- `destroy()` - Stop rendering and cleanup resources


## Multiple Instances

Multiple renderers can run independently on the same page:

```typescript
const config1 = new RendererConfig('terminal-1')
  .withDsl(dsl1)
  .withCanvas(content1);
const renderer1 = createRenderer(config1);

const config2 = new RendererConfig('terminal-2')
  .withDsl(dsl2)
  .withCanvas(content2);
const renderer2 = createRenderer(config2);

renderer1.updateEffect(newDsl);
renderer2.restartEffect();
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

[npm]: https://www.npmjs.com/package/tachyon-fxrenderer
[NPM Badge]: https://img.shields.io/npm/v/tachyonfx-renderer.svg
