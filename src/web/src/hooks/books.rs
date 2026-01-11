use leptos::prelude::{Get, Signal, expect_context};

use crate::{
    contexts::books::{Book, BooksContext},
    hooks::app::use_stage,
};

pub fn use_books() -> BooksContext {
    expect_context::<BooksContext>()
}

pub fn use_library() -> Signal<Vec<Book>> {
    Signal::derive(move || use_books().library.get())
}

pub fn use_staged_book() -> Signal<Option<Book>> {
    let library = use_library();
    let stage = use_stage();

    Signal::derive(move || {
        stage
            .get()
            .and_then(|book_id| library.get().into_iter().find(|book| book.id == book_id))
    })
}
