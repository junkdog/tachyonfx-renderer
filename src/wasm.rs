use std::{
    collections::HashMap,
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicBool, AtomicU32, Ordering},
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

// Instance data stored in registry
struct InstanceData {
    sender: Sender<AppEvent>,
    running: Arc<AtomicBool>,
}

// Registry of instance data
static INSTANCES: LazyLock<Mutex<HashMap<u32, InstanceData>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn next_instance_id() -> u32 {
    NEXT_INSTANCE_ID.fetch_add(1, Ordering::Relaxed)
}

fn register_instance(id: u32, sender: Sender<AppEvent>, running: Arc<AtomicBool>) {
    INSTANCES
        .lock()
        .unwrap()
        .insert(id, InstanceData { sender, running });
}

fn get_sender(id: u32) -> Option<Sender<AppEvent>> {
    INSTANCES
        .lock()
        .unwrap()
        .get(&id)
        .map(|data| data.sender.clone())
}

fn get_running_flag(id: u32) -> Option<Arc<AtomicBool>> {
    INSTANCES
        .lock()
        .unwrap()
        .get(&id)
        .map(|data| data.running.clone())
}

fn remove_instance(id: u32) {
    INSTANCES.lock().unwrap().remove(&id);
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

    // Create running flag (starts as true)
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    // Start render loop with state check
    let mut app = App::new();
    terminal.draw_web(move |frame| {
        // Only render if instance is running
        if !running_clone.load(Ordering::Relaxed) {
            return;
        }

        for event in events.iter() {
            app.apply_event(event);
        }
        app.render(frame);
    });

    // Register instance and return handle
    let instance_id = next_instance_id();
    register_instance(instance_id, sender, running);

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

    pub fn start(&self) {
        if let Some(running) = get_running_flag(self.instance_id) {
            running.store(true, Ordering::Relaxed);
        }
    }

    pub fn stop(&self) {
        if let Some(running) = get_running_flag(self.instance_id) {
            running.store(false, Ordering::Relaxed);
        }
    }

    pub fn is_running(&self) -> bool {
        get_running_flag(self.instance_id)
            .map(|running| running.load(Ordering::Relaxed))
            .unwrap_or(false)
    }

    pub fn destroy(self) {
        // Stop rendering
        if let Some(running) = get_running_flag(self.instance_id) {
            running.store(false, Ordering::Relaxed);
        }

        // Remove from registry
        remove_instance(self.instance_id);

        // Note: Cannot stop the render loop yet - ratzilla doesn't support it
        // The loop will continue but won't render anything since running flag is false
    }
}
