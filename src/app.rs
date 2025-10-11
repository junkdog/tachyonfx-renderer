use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use tachyonfx::{Duration, Effect, EffectManager};
use crate::event::AppEvent;

pub struct App {
    sender: std::sync::mpsc::Sender<AppEvent>,
    effects: EffectManager<u8>,
    last_tick_instant: web_time::Instant,
    last_tick_duration: Duration,
    is_running: bool,
}

impl App {
    pub fn new(sender: std::sync::mpsc::Sender<AppEvent>) -> Self {
        Self {
            sender,
            effects: Default::default(),
            last_tick_instant: web_time::Instant::now(),
            last_tick_duration: Duration::default(),
            is_running: true,
        }
    }

    pub fn tick(&mut self) -> Duration {
        let now = web_time::Instant::now();
        let elapsed = now.duration_since(self.last_tick_instant).as_millis();

        self.last_tick_instant = now;
        self.last_tick_duration = Duration::from_millis(elapsed as u32);

        self.last_tick_duration
    }

    pub fn process_effects(&mut self, last_frame_duration: Duration, buf: &mut Buffer, area: Rect) {
        self.effects.process_effects(last_frame_duration, buf, area);
    }

    pub fn register_effect(&mut self, effect: Effect) {
        self.effects.add_effect(effect);
    }

    pub fn apply_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Tick => (),
            AppEvent::Resize(w, h) => {}
            AppEvent::ReplaceCanvas(ansi) => {}
            AppEvent::CompileDsl(code) => {}
            AppEvent::ReplayCurrentEffect => {}
        }
    }

    pub fn sender(&self) -> std::sync::mpsc::Sender<AppEvent> {
        self.sender.clone()
    }
}
