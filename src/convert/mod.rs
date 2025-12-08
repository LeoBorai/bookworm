pub mod epub;
pub mod pdf;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct BookMetadata {
    pub title: String,
    pub author: String,
    pub subject: Option<String>,
    pub creator: Option<String>,
    pub creation_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct Chapter {
    pub title: String,
    pub content: String,
    pub page_start: usize,
    pub page_end: usize,
}
