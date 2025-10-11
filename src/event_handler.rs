use std::sync::mpsc;
use crate::dispatcher::Dispatcher;
use crate::event::AppEvent;

#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<AppEvent>,
    receiver: mpsc::Receiver<AppEvent>,
}


impl EventHandler {
    pub fn new(_tick_rate: core::time::Duration) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { sender, receiver }
    }

    pub fn sender(&self) -> mpsc::Sender<AppEvent> {
        self.sender.clone()
    }

    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.receiver.recv()
    }

    pub fn try_next(&self) -> Option<AppEvent> {
        match self.receiver.try_recv() {
            Ok(e) => Some(e),
            Err(_) => None
        }
    }
}


impl Dispatcher<AppEvent> for EventHandler {
    fn dispatch(&self, event: AppEvent) {
        let _ = self.sender.send(event);
    }
}
