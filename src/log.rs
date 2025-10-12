use wasm_bindgen::JsValue;

pub fn log_info(msg: impl Into<String>) {
    let msg = msg.into();
    web_sys::console::log_1(&JsValue::from_str(&msg));
}

pub fn log_error(msg: impl Into<String>) {
    let msg = msg.into();
    web_sys::console::error_1(&JsValue::from_str(&msg));
}
