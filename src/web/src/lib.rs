mod components;
mod contexts;
mod hooks;

use leptos::prelude::*;
use leptos_meta::*;

use crate::components::atoms::manager::Manager;
use crate::contexts::app::AppContext;
use crate::contexts::books::BooksContext;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(AppContext::default());
    provide_context(BooksContext::default());

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
