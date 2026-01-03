use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use bookworm::pdf::{Pdf, PdfMetaField};

#[derive(Args, Clone, Debug)]
pub struct SetOpt {
    /// Path to the PDF file
    path: PathBuf,
    /// Sets the `Title` field
    #[clap(long)]
    title: Option<String>,
    /// Sets the `Author` field
    #[clap(long)]
    author: Option<String>,
    /// Sets the `Creator` field
    #[clap(long)]
    creator: Option<String>,
    /// Sets the `Producer` field
    #[clap(long)]
    producer: Option<String>,
    /// Sets the `Creation Date` field
    #[clap(long)]
    creation_date: Option<String>,
    /// Sets the `Modification Date` field
    #[clap(long)]
    modification_date: Option<String>,
}

impl SetOpt {
    pub async fn exec(&self) -> Result<()> {
        let mut pdf = Pdf::open(&self.path)?;

        if let Some(title) = &self.title {
            pdf = pdf.set_metadata(&PdfMetaField::Title, title)?;
        }

        if let Some(author) = &self.author {
            pdf = pdf.set_metadata(&PdfMetaField::Author, author)?;
        }

        if let Some(creator) = &self.creator {
            pdf = pdf.set_metadata(&PdfMetaField::Creator, creator)?;
        }

        if let Some(producer) = &self.producer {
            pdf = pdf.set_metadata(&PdfMetaField::Producer, producer)?;
        }

        if let Some(creation_date) = &self.creation_date {
            pdf = pdf.set_metadata(&PdfMetaField::CreationDate, creation_date)?;
        }

        if let Some(modification_date) = &self.modification_date {
            pdf = pdf.set_metadata(&PdfMetaField::ModificationDate, modification_date)?;
        }

        let next_path = self.path.with_file_name("set_info.pdf");

        pdf.save(next_path)?;

        Ok(())
    }
}
