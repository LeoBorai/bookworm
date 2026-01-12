use anyhow::{Result, anyhow};
use leptos::task::spawn_local;
use leptos::{html, prelude::*};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Event, File, HtmlInputElement};

use bookworm::epub::Epub;
use bookworm::pdf::Pdf;

use crate::contexts::books::Book;

#[component]
pub fn FileUploadButton(#[prop(into)] class: String, children: Children) -> impl IntoView {
    let file_input_el = NodeRef::<html::Input>::new();
    let on_file_change = move |ev: Event| {
        let el = ev
            .target()
            .expect("Failed to retrieve target")
            .unchecked_into::<HtmlInputElement>();

        if let Some(files) = el.files()
            && let Some(file) = files.get(0)
        {
            spawn_local(async move {
                let filename = file.name();
                let array_buffer_promise = file.array_buffer();
                let bytes = read_promise_as_bytes(array_buffer_promise).await.unwrap();
                if filename.ends_with(".pdf") {
                    let pdf = Pdf::from_bytes(&bytes).unwrap();
                    let metadata = pdf.metadata().unwrap();

                    let b = Book {
                        id: 1,
                        title: metadata.title.unwrap_or(String::from("Unknown")),
                        author: metadata.author.unwrap_or(String::from("Unknown")),
                        date: metadata.creation_date.unwrap_or(String::from("Unknown")),
                        keywords: Vec::new(),
                        format: String::from("PDF"),
                        size: bytes.len() as u64,
                    };

                    leptos::logging::log!("test: {:?}", b);
                }

                if filename.ends_with(".epub") {
                    let _epub = Epub::from_bytes(bytes);
                }
            });
        }
    };

    let handle_button_click = {
        move |_| {
            file_input_el.get_untracked().unwrap().click();
        }
    };

    view! {
        <button class=class on:click=handle_button_click>
            <input type="file" hidden=true node_ref=file_input_el on:change=on_file_change />
            {children()}
        </button>
    }
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

async fn read_file_as_bytes(file: File) -> Result<Vec<u8>> {
    let array_buffer_promise = file.array_buffer();
    let array_buffer_value = JsFuture::from(array_buffer_promise)
        .await
        .map_err(|e| anyhow!("Failed to read file: {:?}", e))?;
    let array_buffer = array_buffer_value
        .dyn_into::<js_sys::ArrayBuffer>()
        .map_err(|_| anyhow!("Failed to cast to ArrayBuffer".to_string()))?;

    // Convert ArrayBuffer to Uint8Array
    let uint8_array = js_sys::Uint8Array::new(&array_buffer);

    // Convert to Vec<u8>
    Ok(uint8_array.to_vec())
}
