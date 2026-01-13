use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::File;

use bookworm::epub::Epub;
use bookworm::pdf::Pdf;

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
        log(&format!("Processed: {:?}", data));
        let file: File = File::from(data);

        spawn_local(async move {
            log("Processing file in worker");
            let filename = file.name();
            let array_buffer_promise = file.array_buffer();
            let bytes = read_promise_as_bytes(array_buffer_promise).await.unwrap();
            if filename.ends_with(".pdf") {
                let pdf = Pdf::from_bytes(&bytes).unwrap();
                let metadata = pdf.metadata().unwrap();
                log(&format!(
                    "PDF Metadata - Title: {:?}, Author: {:?}, Creation Date: {:?}",
                    metadata.title, metadata.author, metadata.creation_date
                ));
            }

            if filename.ends_with(".epub") {
                let _epub = Epub::from_bytes(bytes);
            }
        });

        // Send result back to main thread
        // let global = js_sys::global().unchecked_into::<web_sys::DedicatedWorkerGlobalScope>();
        // global.post_message(&JsValue::from_str(&result)).unwrap();
    }) as Box<dyn FnMut(_)>);

    let global = js_sys::global().unchecked_into::<web_sys::DedicatedWorkerGlobalScope>();
    global.set_onmessage(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

async fn read_promise_as_bytes(promise: js_sys::Promise) -> Result<Vec<u8>> {
    let array_buffer_value = JsFuture::from(promise)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read file: {:?}", e))?;
    let array_buffer = array_buffer_value
        .dyn_into::<js_sys::ArrayBuffer>()
        .map_err(|_| anyhow::anyhow!("Failed to cast to ArrayBuffer"))?;
    let uint8_array = js_sys::Uint8Array::new(&array_buffer);

    Ok(uint8_array.to_vec())
}
