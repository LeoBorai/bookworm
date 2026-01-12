use leptos::{html, prelude::*};
use web_sys::{Event, HtmlInputElement, wasm_bindgen::JsCast};

#[component]
pub fn FileUploadButton(#[prop(into)] class: String, children: Children) -> impl IntoView {
    let file_input_el = NodeRef::<html::Input>::new();
    let on_file_change = move |ev: Event| {
        let el = ev
            .target()
            .expect("Failed to retrieve target")
            .unchecked_into::<HtmlInputElement>();

        if let Some(files) = el.files() {
            if let Some(file) = files.get(0) {
                // set_selected_file.set(Some(file.name()));
            }
        }
    };

    let handle_button_click = {
        move |_| {
            file_input_el.get_untracked().unwrap().click();
        }
    };

    view! {
        <button class=class on:click={handle_button_click}>
            <input
                type="file"
                hidden=true
                node_ref=file_input_el
                on:change=on_file_change
            />
            {children()}
        </button>
    }
}
