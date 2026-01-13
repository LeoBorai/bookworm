mod components;
mod contexts;
mod hooks;

use std::cell::RefCell;
use std::rc::Rc;

use leptos::logging::{error, log};
use leptos::prelude::*;
use leptos_meta::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{Worker, WorkerOptions};

use crate::components::atoms::manager::Manager;
use crate::contexts::app::AppContext;
use crate::contexts::books::BooksContext;

thread_local!(pub static WEB_WORKER: Rc<RefCell<Option<Worker>>> = Rc::new(RefCell::new(None)));

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(AppContext::default());
    provide_context(BooksContext::default());

    let _ = RenderEffect::new(move |_| {
        WEB_WORKER.with(|worker_ref| {
            let opts = WorkerOptions::new();
            opts.set_type(web_sys::WorkerType::Module);

            let Ok(worker) = Worker::new_with_options("/web-worker-init.js", &opts) else {
                error!("Failed to create web worker.");
                return;
            };
            let closure = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
                log!("Received message from worker");
                let data = event.data();
                if let Some(result_str) = data.as_string() {
                    leptos::logging::log!("{result_str}");
                }
            }) as Box<dyn FnMut(_)>);

            worker.set_onmessage(Some(closure.as_ref().unchecked_ref()));
            closure.forget();

            log!("Set worker");
            worker_ref.borrow_mut().replace(worker);
        });
    });

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        <Title text="Bookworm | Utilities to manage your ebook collection (PDFs, ePubs, KePubs, and more)" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>
            <Manager />
        </ErrorBoundary>
    }
}
