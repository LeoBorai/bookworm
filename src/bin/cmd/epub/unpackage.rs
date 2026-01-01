use std::path::PathBuf;

use clap::Args;

use bookworm::epub::Epub;

#[derive(Args, Clone, Debug)]
pub struct UnPackageOpt {
    /// Path to the (K)Epub file
    path: PathBuf,
    /// Directory to unpackage the (K)Epub file into
    #[clap(long, short)]
    output: Option<PathBuf>,
}

impl UnPackageOpt {
    pub async fn exec(&self) -> anyhow::Result<()> {
        let outdir = match &self.output {
            Some(dir) => dir.clone(),
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
                parent.join(file_stem)
            }
        };

        Epub::unpackage(&self.path, &outdir)?;

        Ok(())
    }
}
