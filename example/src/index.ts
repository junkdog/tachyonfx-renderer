// Dynamic imports for WASM module
type TachyonModule = typeof import('tachyonfx-renderer');

let createRenderer: TachyonModule['createRenderer'];
let TachyonRenderer: TachyonModule['TachyonRenderer'];

async function initWasm() {
  const module = await import('tachyonfx-renderer');
  await module.default();
  createRenderer = module.createRenderer;
  TachyonRenderer = module.TachyonRenderer;
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

async function loadAnsiFile(path: string): Promise<string> {
  const response = await fetch(path);
  return await response.text();
}

class RendererDemo {
  private renderer1: any = null;
  private renderer2: any = null;
  private canvas1Content: string = '';
  private canvas2Content: string = '';

  async initialize() {
    // Initialize WASM module
    await initWasm();

    // Load ANSI content
    this.canvas1Content = await loadAnsiFile('default_canvas.ansi');
    this.canvas2Content = await loadAnsiFile('key-press-fx.ansi');

    // Create both renderers
    this.renderer1 = createRenderer('canvas1', SLIDE_IN_EFFECT, this.canvas1Content);
    this.renderer2 = createRenderer('canvas2', SWEEP_EFFECT, this.canvas2Content);

    this.setupControls();
    this.logStatus();
  }

  private setupControls() {
    // Canvas 1 controls
    this.setupButton('btn-canvas1-stop', () => {
      this.renderer1?.stop();
      this.logStatus();
    });

    this.setupButton('btn-canvas1-start', () => {
      this.renderer1?.start();
      this.logStatus();
    });

    this.setupButton('btn-canvas1-replay', () => {
      this.renderer1?.replayEffect();
    });

    this.setupButton('btn-canvas1-effect-fade', () => {
      this.renderer1?.updateEffect(FADE_EFFECT);
    });

    this.setupButton('btn-canvas1-effect-complex', () => {
      this.renderer1?.updateEffect(COMPLEX_EFFECT);
    });

    // Canvas 2 controls
    this.setupButton('btn-canvas2-stop', () => {
      this.renderer2?.stop();
      this.logStatus();
    });

    this.setupButton('btn-canvas2-start', () => {
      this.renderer2?.start();
      this.logStatus();
    });

    this.setupButton('btn-canvas2-replay', () => {
      this.renderer2?.replayEffect();
    });

    this.setupButton('btn-canvas2-swap', () => {
      // Swap content between canvases
      const temp = this.canvas1Content;
      this.canvas1Content = this.canvas2Content;
      this.canvas2Content = temp;

      this.renderer2?.updateCanvas(this.canvas2Content);
    });

    // Global controls
    this.setupButton('btn-stop-all', () => {
      this.renderer1?.stop();
      this.renderer2?.stop();
      this.logStatus();
    });

    this.setupButton('btn-start-all', () => {
      this.renderer1?.start();
      this.renderer2?.start();
      this.logStatus();
    });

    this.setupButton('btn-destroy-all', () => {
      this.renderer1?.destroy();
      this.renderer2?.destroy();
      this.renderer1 = null;
      this.renderer2 = null;
      this.logStatus();
      this.disableAllButtons();
    });
  }

  private setupButton(id: string, callback: () => void) {
    const button = document.getElementById(id);
    if (button) {
      button.addEventListener('click', callback);
    }
  }

  private logStatus() {
    const status1 = this.renderer1?.isRunning() ?? false;
    const status2 = this.renderer2?.isRunning() ?? false;

    console.log('Renderer Status:', {
      canvas1: status1 ? 'running' : 'stopped',
      canvas2: status2 ? 'running' : 'stopped',
    });

    this.updateStatusDisplay(status1, status2);
  }

  private updateStatusDisplay(status1: boolean, status2: boolean) {
    const statusEl1 = document.getElementById('status1');
    const statusEl2 = document.getElementById('status2');

    if (statusEl1) {
      statusEl1.textContent = status1 ? '🟢 Running' : '🔴 Stopped';
      statusEl1.className = status1 ? 'status running' : 'status stopped';
    }

    if (statusEl2) {
      statusEl2.textContent = status2 ? '🟢 Running' : '🔴 Stopped';
      statusEl2.className = status2 ? 'status running' : 'status stopped';
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
