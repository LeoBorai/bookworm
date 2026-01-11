use leptos::prelude::{Get, Signal, expect_context};

use crate::contexts::app::AppContext;

pub fn use_app() -> AppContext {
    expect_context::<AppContext>()
}

pub fn use_stage() -> Signal<Option<u32>> {
    Signal::derive(move || use_app().stage.get())
}

pub fn use_set_stage() -> impl Fn(u32) {
    let app = use_app();
    move |book_id: u32| {
        app.set_stage(book_id);
    }
}
