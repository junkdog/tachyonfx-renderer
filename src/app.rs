use tachyonfx::{Duration, EffectManager};
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

    pub fn sender(&self) -> std::sync::mpsc::Sender<AppEvent> {
        self.sender.clone()
    }
}
