use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use bookworm::pdf::Pdf;

#[derive(Args, Clone, Debug)]
pub struct GetOpt {
    /// Path to the PDF file
    path: PathBuf,
}

impl GetOpt {
    pub async fn exec(&self) -> Result<()> {
        let pdf = Pdf::open(&self.path)?;
        let info = pdf.metadata()?;

        println!(
            "Title: {}",
            info.title.unwrap_or_else(|| "Unknown".to_string())
        );
        println!(
            "Author: {}",
            info.author.unwrap_or_else(|| "Unknown".to_string())
        );
        println!(
            "Creator: {}",
            info.creator.unwrap_or_else(|| "Unknown".to_string())
        );
        println!(
            "Producer: {}",
            info.producer.unwrap_or_else(|| "Unknown".to_string())
        );
        println!(
            "Creation Date: {}",
            info.creation_date.unwrap_or_else(|| "Unknown".to_string())
        );
        println!(
            "Modification Date: {}",
            info.modification_date
                .unwrap_or_else(|| "Unknown".to_string())
        );

        Ok(())
    }
}
