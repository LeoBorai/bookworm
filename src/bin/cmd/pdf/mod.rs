mod meta;

use anyhow::Result;
use clap::Subcommand;

use self::meta::MetadataCmd;

#[derive(Clone, Debug, Subcommand)]
pub enum PdfCmd {
    #[clap(subcommand)]
    /// Retrieve PDF file metadata
    Metadata(MetadataCmd),
}

impl PdfCmd {
    pub async fn exec(&self) -> Result<()> {
        match self {
            Self::Metadata(cmd) => cmd.exec().await,
        }
    }
}
