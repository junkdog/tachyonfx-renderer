use std::{
    collections::HashMap,
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicU32, Ordering},
        mpsc::Sender,
    },
};

use ratzilla::WebRenderer;
use wasm_bindgen::prelude::*;

use crate::{
    app::App,
    dispatcher::Dispatcher,
    event::AppEvent::{self, *},
    event_handler::EventHandler,
    terminal::create_terminal,
};

// Global instance ID counter
static NEXT_INSTANCE_ID: AtomicU32 = AtomicU32::new(0);

// Registry of event senders for each instance
static SENDERS: LazyLock<Mutex<HashMap<u32, Sender<AppEvent>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn next_instance_id() -> u32 {
    NEXT_INSTANCE_ID.fetch_add(1, Ordering::Relaxed)
}

fn register_sender(id: u32, sender: Sender<AppEvent>) {
    SENDERS.lock().unwrap().insert(id, sender);
}

fn get_sender(id: u32) -> Option<Sender<AppEvent>> {
    SENDERS.lock().unwrap().get(&id).cloned()
}

fn remove_sender(id: u32) {
    SENDERS.lock().unwrap().remove(&id);
}

#[wasm_bindgen]
pub struct TachyonRenderer {
    instance_id: u32,
}

#[wasm_bindgen]
pub fn create_renderer(
    container_id: &str,
    dsl_code: &str,
    canvas_content: &str,
) -> Result<TachyonRenderer, JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let events = EventHandler::new(core::time::Duration::from_millis(33));
    let sender = events.sender();

    // Send initial events to set up canvas and effect
    sender.dispatch(ReplaceCanvas(canvas_content.into()));
    sender.dispatch(CompileDsl(dsl_code.into()));

    // Create terminal for this container
    let terminal = create_terminal(container_id)
        .map_err(|e| JsValue::from_str(&format!("Failed to create terminal: {}", e)))?;

    // Start render loop
    let mut app = App::new();
    terminal.draw_web(move |frame| {
        for event in events.iter() {
            app.apply_event(event);
        }
        app.render(frame);
    });

    // Register instance and return handle
    let instance_id = next_instance_id();
    register_sender(instance_id, sender);

    Ok(TachyonRenderer { instance_id })
}

#[wasm_bindgen]
impl TachyonRenderer {
    pub fn update_canvas(&self, ansi_content: &str) {
        if let Some(sender) = get_sender(self.instance_id) {
            sender.dispatch(ReplaceCanvas(ansi_content.into()));
        }
    }

    pub fn update_effect(&self, dsl_code: &str) {
        if let Some(sender) = get_sender(self.instance_id) {
            sender.dispatch(CompileDsl(dsl_code.into()));
        }
    }

    pub fn destroy(self) {
        remove_sender(self.instance_id);
        // Note: Cannot stop the render loop yet - ratzilla doesn't support it
        // The loop will continue but updates will be ignored since sender is removed
    }
}
