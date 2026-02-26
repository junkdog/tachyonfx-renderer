## tachyonfx-renderer 0.4.0 - 2026-02-26

### Features
- `withDynamicFontAtlas(fontFamilies, fontSize)` - Configure a dynamic font atlas with custom font families and size, enabling proper glyph rendering for user-specified fonts
- Post-creation canvas resize when using dynamic font atlas, ensuring correct dimensions once cell size is known

### Changes
- `withAutoResizeCanvasCss` adapted to upstream ratzilla API change (`disable_auto_css_resize()`)

### Dependency Updates
- ratzilla: `junkdog/beamterm-0.16.0` branch
- tachyonfx: 0.25.0 pre-release (devevelopment branch)
- beamterm 0.16.0

## tachyonfx-renderer 0.3.0 - 2026-02-07

### Features
- `withCanvasPaddingColor(color)` - Set the background color for canvas padding area (hex RGB, e.g. `0x1d2021`)
- `withAutoResizeCanvasCss(enable)` - Control whether the renderer auto-sets canvas CSS dimensions on resize

### Dependency Updates
- tachyonfx 0.23.0
- ratzilla 0.3.0
- beamterm 0.14.0

## tachyonfx-renderer 0.2.1 - 2025-10-18

### Dependency Updates
- ratatui 0.3.0/ratzilla 0.3.0

## tachyonfx-renderer 0.2.1 - 2025-10-18
- use tachyonfx 0.20.1

## tachyonfx-renderer 0.2.0 - 2025-10-18

### Breaking Changes
- **API redesign**: `createRenderer` now accepts a `RendererConfig` object instead of multiple parameters
  - Old: `createRenderer(containerId, dslCode, ansiContent, sleepMs?)`
  - New: `createRenderer(config)` where config is built with:
    ```typescript
    new RendererConfig(containerId)
      .withDsl(dslCode)
      .withCanvas(ansiContent)
      .withSleepBetweenReplay(sleepMs)  // optional
    ```
- **Method rename**: `playEffect()` renamed to `restartEffect()`

### Features
- Make effects loop automatically with configurable sleep duration


## tachyonfx-renderer 0.1.0 - 2025-10-14
- Initial release

