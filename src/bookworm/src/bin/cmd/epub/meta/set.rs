use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use bookworm::epub::{Epub, EpubMetaField};

#[derive(Args, Clone, Debug)]
pub struct SetOpt {
    /// Path to the EPUB file
    path: PathBuf,
    /// Sets the `Title` field
    #[clap(long)]
    title: Option<String>,
    /// Sets the `Creator` (author) field
    #[clap(long)]
    creator: Option<String>,
    /// Sets the `Language` field
    #[clap(long)]
    language: Option<String>,
    /// Sets the `Identifier` field
    #[clap(long)]
    identifier: Option<String>,
}

impl SetOpt {
    pub async fn exec(&self) -> Result<()> {
        let mut epub = Epub::open(&self.path)?;

        if let Some(title) = &self.title {
            epub.set_metadata(&EpubMetaField::Title, title)?;
        }

        if let Some(creator) = &self.creator {
            epub.set_metadata(&EpubMetaField::Creator, creator)?;
        }

        if let Some(language) = &self.language {
            epub.set_metadata(&EpubMetaField::Language, language)?;
        }

        if let Some(identifier) = &self.identifier {
            epub.set_metadata(&EpubMetaField::Identifier, identifier)?;
        }

        let next_path = self.path.with_file_name("set_info.epub");

        epub.save(next_path)?;

        Ok(())
    }
}
