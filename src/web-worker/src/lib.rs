use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
pub struct WorkerMessage {
    pub data: String,
}

#[wasm_bindgen]
pub fn worker_entry_point() {
    log("Initializing worker");

    // Send ready signal
    let global = js_sys::global().unchecked_into::<web_sys::DedicatedWorkerGlobalScope>();
    global.post_message(&JsValue::from_str("READY")).unwrap();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
        log("Received event");
        let data = event.data();

        // Process data (example: compute-heavy task)
        let result = format!("Processed: {:?}", data);
        log(result.as_str());
        // Send result back to main thread
        let global = js_sys::global().unchecked_into::<web_sys::DedicatedWorkerGlobalScope>();
        global.post_message(&JsValue::from_str(&result)).unwrap();
    }) as Box<dyn FnMut(_)>);

    let global = js_sys::global().unchecked_into::<web_sys::DedicatedWorkerGlobalScope>();
    global.set_onmessage(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}
