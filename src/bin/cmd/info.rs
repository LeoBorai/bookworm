use std::fs::rename;
use std::path::PathBuf;

use clap::Args;

use kepub::epub::Epub;

#[derive(Args, Clone, Debug)]
pub struct InfoCmd {
    /// Path to the (K)Epub file
    path: PathBuf,
    /// Renames the (K)Epub file
    #[clap(long)]
    rename: bool,
}

impl InfoCmd {
    pub async fn exec(&self) -> anyhow::Result<()> {
        let epub = Epub::open(&self.path)?;
        let content_opf = epub.content_opf();

        println!("Title: {}", content_opf.metadata.title);
        println!("Author: {}", content_opf.metadata.creator);
        println!("Language: {}", content_opf.metadata.language);
        println!("Identifier: {}", content_opf.metadata.identifier);

        if self.rename {
            let parent = self
                .path
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory"))?;
            let extension = self
                .path
                .extension()
                .and_then(|ext| ext.to_str())
                .ok_or_else(|| anyhow::anyhow!("Failed to get file extension"))?;
            let safe_title = content_opf
                .metadata
                .title
                .replace("/", "-")
                .replace("\\", "-");
            let safe_creator = content_opf
                .metadata
                .creator
                .replace("/", "-")
                .replace("\\", "-");
            let new_file_name = format!("{} - {}.{}", safe_title, safe_creator, extension);
            let new_path = parent.join(new_file_name);

            rename(&self.path, &new_path)?;

            println!("Renamed file to: {}", new_path.display());
        }

        Ok(())
    }
}
