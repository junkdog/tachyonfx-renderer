## tachyonfx-renderer 0.2.0 - TBD

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

### Features
- Make effects loop automatically with configurable sleep duration

## tachyonfx-renderer 0.1.0 - 2025-10-14
- Initial release

