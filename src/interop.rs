use std::sync::mpsc::Sender;
use wasm_bindgen::prelude::*;
use crate::dispatcher::Dispatcher;
use crate::event::AppEvent::{self, *};

#[wasm_bindgen]
pub fn compile_dsl(s: &str) {
    sender().dispatch(CompileDsl(s.into()));
}

#[wasm_bindgen]
pub fn update_canvas(s: &str) {
    sender().dispatch(ReplaceCanvas(s.into()));
}


fn sender() -> Sender<AppEvent> {
    unsafe {
        #[allow(static_mut_refs)] // init_global_state only called once at startup
        match SENDER.as_ref() {
            None => panic!("No sender in global state"),
            Some(s) => s.event_sender.clone(),
        }
    }
}

struct JsSender {
    event_sender: Sender<AppEvent>,
}

pub fn init_global_state(sender: Sender<AppEvent>) {
    unsafe {
        SENDER = Some(JsSender {
            event_sender: sender,
        });
    }
}

static mut SENDER: Option<JsSender> = None;
