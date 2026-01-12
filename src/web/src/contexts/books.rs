use leptos::prelude::RwSignal;

#[derive(Clone, Debug)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub date: String,
    pub keywords: Vec<String>,
    pub format: String,
    pub size: u64,
}

#[derive(Clone, Debug)]
pub struct BooksContext {
    pub library: RwSignal<Vec<Book>>,
}

impl Default for BooksContext {
    fn default() -> Self {
        Self {
            library: RwSignal::new(Vec::default()),
        }
    }
}

impl BooksContext {}
