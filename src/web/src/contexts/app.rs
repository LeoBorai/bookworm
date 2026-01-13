use leptos::prelude::{RwSignal, Set};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub stage: RwSignal<Option<u32>>,
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            stage: RwSignal::new(None),
        }
    }
}

impl AppContext {
    pub fn set_stage(&self, book_id: u32) {
        self.stage.set(Some(book_id));
    }
}
