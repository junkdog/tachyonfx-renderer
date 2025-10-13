use ansi_to_tui::IntoText;
use eyre::Result;
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Offset, Rect},
    widgets::Widget,
};
use tachyonfx::{Duration, Effect, EffectManager, blit_buffer, dsl::EffectDsl};

use crate::{event::AppEvent, log::log_error};

pub struct App {
    effects: EffectManager<u8>,
    effect_dsl: Option<String>,
    canvas_buf: Buffer,
    last_tick_instant: web_time::Instant,
    last_tick_duration: Duration,
}

impl App {
    pub fn new() -> Self {
        let area = Rect::new(0, 0, 20, 10);
        Self {
            effects: Default::default(),
            canvas_buf: Buffer::empty(area),
            last_tick_instant: web_time::Instant::now(),
            last_tick_duration: Duration::default(),
            effect_dsl: None,
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let elapsed = self.tick();

        // copy canvas to frame buffer
        blit_buffer(&self.canvas_buf, frame.buffer_mut(), Offset::default());

        let area = frame.area();
        self.effects
            .process_effects(elapsed, frame.buffer_mut(), area);
    }

    fn register_effect(&mut self, effect: Effect) {
        self.effects.add_unique_effect(0, effect);
    }

    fn replay_effect(&mut self) {
        if let Some(dsl) = &self.effect_dsl {
            let effect = compile_dsl(dsl).expect("Known good DSL should compile successfully");

            self.register_effect(effect);
        }
    }

    pub fn apply_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::ReplaceCanvas(ansi) => self.update_canvas(ansi),
            AppEvent::CompileDsl(code) => self.compile_and_register_effect(code),
            AppEvent::ReplayCurrentEffect => self.replay_effect(),
        }
    }

    fn update_canvas(&mut self, source: String) {
        let Ok(canvas) = source.into_text() else {
            log_error("Failed to parse ANSI input");
            return;
        };

        let h = canvas.lines.len();
        let w = canvas
            .lines
            .iter()
            .map(|line| line.width())
            .max()
            .unwrap_or(0);

        let area = Rect::new(0, 0, w as u16, h as u16);
        self.canvas_buf = Buffer::empty(area);
        canvas.render(area, &mut self.canvas_buf);
    }

    fn compile_and_register_effect(&mut self, code: String) {
        match compile_dsl(&code) {
            Ok(effect) => {
                self.effect_dsl = Some(code); // in case of replay
                self.register_effect(effect)
            },
            Err(e) => log_error(format!("DSL compilation error:\n{}", e)),
        }
    }

    fn tick(&mut self) -> Duration {
        let now = web_time::Instant::now();
        let elapsed = now
            .duration_since(self.last_tick_instant)
            .as_millis();

        self.last_tick_instant = now;
        self.last_tick_duration = Duration::from_millis(elapsed as u32);

        self.last_tick_duration
    }
}

fn compile_dsl(dsl: &str) -> Result<Effect> {
    EffectDsl::new()
        .compiler()
        .compile(dsl)
        .map_err(|e| e.into())
}
