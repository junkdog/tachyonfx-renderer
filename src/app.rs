use ansi_to_tui::IntoText;
use ratatui::buffer::Buffer;
use ratatui::layout::{Offset, Rect};
use ratatui::widgets::Widget;
use ratatui::Frame;
use tachyonfx::{blit_buffer, BufferRenderer, Duration, Effect, EffectManager};
use tachyonfx::dsl::EffectDsl;
use wasm_bindgen::JsValue;
use crate::event::AppEvent;

pub struct App {
    effects: EffectManager<u8>,
    canvas_buf: Buffer,
    last_tick_instant: web_time::Instant,
    last_tick_duration: Duration,
    sender: std::sync::mpsc::Sender<AppEvent>,
}

impl App {
    pub fn new(sender: std::sync::mpsc::Sender<AppEvent>) -> Self {
        let area = Rect::new(0, 0, 20, 10);
        Self {
            sender,
            effects: Default::default(),
            canvas_buf: Buffer::empty(area),
            last_tick_instant: web_time::Instant::now(),
            last_tick_duration: Duration::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let elapsed = self.tick();

        // copy canvas to frame buffer
        blit_buffer(&self.canvas_buf, &mut frame.buffer_mut(), Offset::default());

        let area = frame.area();
        self.effects.process_effects(elapsed, frame.buffer_mut(), area);
    }

    pub fn register_effect(&mut self, effect: Effect) {
        self.effects.add_unique_effect(0, effect);
    }

    pub fn apply_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Tick => (),
            AppEvent::Resize(_w, _h) => {}
            AppEvent::ReplaceCanvas(ansi) => self.update_canvas(ansi),
            AppEvent::CompileDsl(code) => self.compile_dsl(code),
            AppEvent::ReplayCurrentEffect => {}
        }
    }

    fn resize_canvas(&mut self, area: Rect) {
        self.canvas_buf = Buffer::empty(area);
    }

    fn update_canvas(&mut self, source: String) {
        let Ok(text) = source.into_text() else {
            web_sys::console::error_1(&JsValue::from_str("Failed to parse ANSI input"));
            return;
        };

        let w = text
            .lines
            .iter()
            .map(|line| line.width())
            .max()
            .unwrap_or(0);
        let h = text.lines.len();

        let area = Rect::new(0, 0, w as u16, h as u16);
        self.resize_canvas(area);

        text.render(area, &mut self.canvas_buf);
    }

    fn compile_dsl(&mut self, dsl: String) {
        let effect = EffectDsl::new().compiler().compile(dsl.as_str());

        match effect {
            Ok(effect) => self.register_effect(effect),
            Err(e) => {
                eprintln!("DSL compilation error: {}", e);
                eprintln!("Context: {}", e.context());
                eprintln!(
                    "Position: line {}, column {}",
                    e.start_line(),
                    e.start_column()
                );
            }
        }
    }

    pub fn sender(&self) -> std::sync::mpsc::Sender<AppEvent> {
        self.sender.clone()
    }

    fn tick(&mut self) -> Duration {
        let now = web_time::Instant::now();
        let elapsed = now.duration_since(self.last_tick_instant).as_millis();

        self.last_tick_instant = now;
        self.last_tick_duration = Duration::from_millis(elapsed as u32);

        self.last_tick_duration
    }
}
