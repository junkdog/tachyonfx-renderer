use std::{
    collections::HashMap,
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicBool, AtomicU32, Ordering},
        mpsc::Sender,
    },
};

use ansi_to_tui::IntoText;
use ratatui::style::Color;
use ratzilla::{
    WebRenderer,
    backend::webgl2::{FontAtlasConfig, FontAtlasData, WebGl2BackendOptions},
};
use tachyonfx::Duration;
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
pub struct RendererConfig {
    container_id: String,
    dsl_code: String,
    canvas_content: String,
    sleep_ms_between_replay: Option<u32>,
    font_families: Option<Vec<String>>,
    font_size: Option<f32>,
    canvas_padding_color: Option<u32>,
    auto_resize_canvas_css: Option<bool>,
}

#[wasm_bindgen]
impl RendererConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(container_id: String) -> Self {
        Self {
            container_id,
            dsl_code: String::new(),
            canvas_content: String::new(),
            sleep_ms_between_replay: None,
            font_families: None,
            font_size: None,
            canvas_padding_color: None,
            auto_resize_canvas_css: None,
        }
    }

    #[wasm_bindgen(js_name = withDsl)]
    pub fn with_dsl(mut self, dsl_code: String) -> Self {
        self.dsl_code = dsl_code;
        self
    }

    #[wasm_bindgen(js_name = withCanvas)]
    pub fn with_canvas(mut self, canvas_content: String) -> Self {
        self.canvas_content = canvas_content;
        self
    }

    #[wasm_bindgen(js_name = withSleepBetweenReplay)]
    pub fn with_sleep_between_replay(mut self, sleep_ms: u32) -> Self {
        self.sleep_ms_between_replay = Some(sleep_ms);
        self
    }

    // TODO: enable once ratzilla exposes cell size query for dynamic atlas
    // #[wasm_bindgen(js_name = withDynamicFontAtlas)]
    // pub fn with_dynamic_font_atlas(mut self, font_families: js_sys::Array, font_size: f32)
    // -> Self {     let families: Vec<String> = font_families
    //         .iter()
    //         .filter_map(|v| v.as_string())
    //         .collect();
    //     self.font_families = Some(families);
    //     self.font_size = Some(font_size);
    //     self
    // }

    #[wasm_bindgen(js_name = withCanvasPaddingColor)]
    pub fn with_canvas_padding_color(mut self, color: u32) -> Self {
        self.canvas_padding_color = Some(color);
        self
    }

    #[wasm_bindgen(js_name = withAutoResizeCanvasCss)]
    pub fn with_auto_resize_canvas_css(mut self, enable: bool) -> Self {
        self.auto_resize_canvas_css = Some(enable);
        self
    }
}

#[wasm_bindgen]
pub struct TachyonFxRenderer {
    instance_id: u32,
}

#[wasm_bindgen(js_name = createRenderer)]
pub fn create_renderer(config: RendererConfig) -> Result<TachyonFxRenderer, JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let events = EventHandler::new(core::time::Duration::from_millis(33));
    let sender = events.sender();

    // Send initial events to set up canvas and effect
    sender.dispatch(ReplaceCanvas(config.canvas_content.clone()));
    sender.dispatch(CompileDsl(config.dsl_code.clone()));

    // Build backend options
    let options = build_backend_options(&config)?;

    // Create terminal with configured options
    let terminal = create_terminal(options)
        .map_err(|e| JsValue::from_str(&format!("Failed to create terminal: {}", e)))?;

    // Create running flag (starts as true)
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let mut app = if let Some(ms) = config.sleep_ms_between_replay {
        App::new().sleep_between_replay(Duration::from_millis(ms))
    } else {
        App::new()
    };

    // Start render loop with state check
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

    Ok(TachyonFxRenderer { instance_id })
}

fn build_backend_options(config: &RendererConfig) -> Result<WebGl2BackendOptions, JsValue> {
    let mut options = WebGl2BackendOptions::new().grid_id(&config.container_id);

    // Calculate canvas size from content dimensions
    let terminal_size = calculate_terminal_size(&config.canvas_content)?;
    options = options.size(calculate_canvas_size(terminal_size));

    // Font atlas configuration
    if let Some(ref families) = config.font_families {
        let font_size = config.font_size.unwrap_or(16.0);
        let family_refs: Vec<&str> = families.iter().map(|s| s.as_str()).collect();
        options = options.font_atlas_config(FontAtlasConfig::dynamic(&family_refs, font_size));
    }

    if let Some(hex) = config.canvas_padding_color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        options = options.canvas_padding_color(Color::Rgb(r, g, b));
    }

    if let Some(auto_resize) = config.auto_resize_canvas_css {
        options = options.auto_resize_canvas_css(auto_resize);
    }

    Ok(options)
}

#[wasm_bindgen]
impl TachyonFxRenderer {
    #[wasm_bindgen(js_name = updateEffect)]
    pub fn update_effect(&self, dsl_code: &str) {
        if let Some(sender) = get_sender(self.instance_id) {
            sender.dispatch(CompileDsl(dsl_code.into()));
        }
    }

    #[wasm_bindgen(js_name = restartEffect)]
    pub fn restart_effect(&self) {
        if let Some(sender) = get_sender(self.instance_id) {
            sender.dispatch(RestartEffect);
        }
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

fn calculate_terminal_size(canvas: &str) -> Result<(u16, u16), JsValue> {
    let text = canvas
        .into_text()
        .map_err(|e| JsValue::from_str(&format!("Failed to parse ANSI input: {}", e)))?;

    let rows = text.lines.len();
    let cols = text
        .lines
        .iter()
        .map(|line| line.width())
        .max()
        .unwrap_or(0);

    Ok((cols as u16, rows as u16))
}

fn calculate_canvas_size(terminal_size: (u16, u16)) -> (u32, u32) {
    let (w, h) = FontAtlasData::default().cell_size;
    (
        terminal_size.0 as u32 * (w as u32 - 2),
        terminal_size.1 as u32 * (h as u32 - 2),
    )
}
