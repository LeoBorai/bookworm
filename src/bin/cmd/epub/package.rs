use std::{fs::File, path::PathBuf};

use clap::Args;

use bookworm::epub::EpubWriter;

#[derive(Args, Clone, Debug)]
pub struct PackageOpt {
    /// Path to the directory to package into a (K)Epub file
    path: PathBuf,
    /// File to package the (K)Epub file into
    #[clap(long, short)]
    output: Option<PathBuf>,
}

impl PackageOpt {
    pub async fn exec(&self) -> anyhow::Result<()> {
        let outfile = match &self.output {
            Some(filename) => filename.clone(),
            None => {
                let parent = self
                    .path
                    .parent()
                    .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory"))?;
                let file_stem = self
                    .path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .ok_or_else(|| anyhow::anyhow!("Failed to get file stem"))?;

                parent.join(file_stem).with_extension("epub")
            }
        };

        let outfile = File::create(&outfile)?;
        let mut epub_writer = EpubWriter::new(outfile, &self.path)?;

        epub_writer.write().await?;

        Ok(())
    }
}
