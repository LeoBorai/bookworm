use std::rc::Rc;

use leptos::{html, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

use crate::WEB_WORKER;

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
            WEB_WORKER.with(Rc::clone).borrow().as_ref().map(|worker| {
                leptos::logging::log!("Got worker");
                if let Err(err) = worker.post_message(&file.into()) {
                    leptos::logging::log!("Failed to send message to worker: {:?}", err);
                }
            });

            // spawn_local(async move {
            //     let filename = file.name();
            //     let array_buffer_promise = file.array_buffer();
            //     let bytes = read_promise_as_bytes(array_buffer_promise).await.unwrap();
            //     if filename.ends_with(".pdf") {
            //         let pdf = Pdf::from_bytes(&bytes).unwrap();
            //         let metadata = pdf.metadata().unwrap();

            //         let b = Book {
            //             id: 1,
            //             title: metadata.title.unwrap_or(String::from("Unknown")),
            //             author: metadata.author.unwrap_or(String::from("Unknown")),
            //             date: metadata.creation_date.unwrap_or(String::from("Unknown")),
            //             keywords: Vec::new(),
            //             format: String::from("PDF"),
            //             size: bytes.len() as u64,
            //         };

            //         leptos::logging::log!("test: {:?}", b);
            //     }

            //     if filename.ends_with(".epub") {
            //         let _epub = Epub::from_bytes(bytes);
            //     }
            // });
        }
    };

    let handle_click = move |_| {
        if let Some(input_el) = file_input_el.get() {
            input_el.click();
        }
    };

    view! {
        <button class=class on:click=handle_click>
            <input type="file" hidden=true node_ref=file_input_el on:change=on_file_change />
            {children()}
        </button>
    }
}
