import { DEFAULT_CANVAS_ANSI, KEY_PRESS_FX_ANSI } from './ansi-content';

// Dynamic imports for WASM module
type TachyonModule = typeof import('tachyonfx-renderer');

let createRenderer: TachyonModule['createRenderer'];
let RendererConfig: TachyonModule['RendererConfig'];
let TachyonRenderer: TachyonModule['TachyonFxRenderer'];

async function initWasm() {
  const module = await import('tachyonfx-renderer');
  await module.default();
  createRenderer = module.createRenderer;
  RendererConfig = module.RendererConfig;
  TachyonRenderer = module.TachyonFxRenderer;
}

// DSL effect examples
const SLIDE_IN_EFFECT = `
  fx::slide_in(Motion::RightToLeft, 10, 0, Color::Black, (800, Interpolation::QuadOut))
`;

const SWEEP_EFFECT = `
  fx::sweep_in(Motion::LeftToRight, 10, 0, Color::Black, (1200, Interpolation::QuadOut))
`;

const FADE_EFFECT = `
  fx::fade_from_fg(Color::Black, (600, Interpolation::CubicOut))
`;

const COMPLEX_EFFECT = `
  fx::parallel(&[
    fx::sweep_in(Motion::RightToLeft, 15, 0, Color::Black, (1000, Interpolation::BounceOut)),
    fx::coalesce((1000, Interpolation::QuadOut))
  ])
`;

class RendererDemo {
  private renderer1: any = null;
  private renderer2: any = null;

  async initialize() {
    // Initialize WASM module
    await initWasm();

    // Create both renderers using config-based API
    const config1 = new RendererConfig('canvas1')
      .withDsl(SLIDE_IN_EFFECT)
      .withCanvas(DEFAULT_CANVAS_ANSI)
      .withSleepBetweenReplay(1500);
    this.renderer1 = createRenderer(config1);

    const config2 = new RendererConfig('canvas2')
      .withDsl(SWEEP_EFFECT)
      .withCanvas(KEY_PRESS_FX_ANSI);
    this.renderer2 = createRenderer(config2);

    this.setupControls();
  }

  private setupControls() {
    // Canvas 1 controls
    this.setupButton('btn-canvas1-replay', () => {
      this.renderer1?.playEffect();
    });

    this.setupButton('btn-canvas1-effect-fade', () => {
      this.renderer1?.updateEffect(FADE_EFFECT);
    });

    this.setupButton('btn-canvas1-effect-complex', () => {
      this.renderer1?.updateEffect(COMPLEX_EFFECT);
    });

    // Canvas 2 controls
    this.setupButton('btn-canvas2-replay', () => {
      this.renderer2?.playEffect();
    });

    // Global controls
    this.setupButton('btn-destroy-all', () => {
      this.renderer1?.destroy();
      this.renderer2?.destroy();
      this.renderer1 = null;
      this.renderer2 = null;
      this.disableAllButtons();
    });
  }

  private setupButton(id: string, callback: () => void) {
    const button = document.getElementById(id);
    if (button) {
      button.addEventListener('click', callback);
    }
  }

  private disableAllButtons() {
    const buttons = document.querySelectorAll('button');
    buttons.forEach(btn => {
      if (btn.id !== 'btn-destroy-all') {
        (btn as HTMLButtonElement).disabled = true;
      }
    });
  }
}

// Start the demo when page loads
window.addEventListener('DOMContentLoaded', async () => {
  const demo = new RendererDemo();
  await demo.initialize();
});
