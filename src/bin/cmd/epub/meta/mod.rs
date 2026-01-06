mod get;

use anyhow::Result;
use clap::Subcommand;

use crate::cmd::epub::meta::get::GetOpt;

#[derive(Clone, Debug, Subcommand)]
pub enum MetadataCmd {
    /// Retrieve EPUB file metadata
    Get(GetOpt),
}

impl MetadataCmd {
    pub async fn exec(&self) -> Result<()> {
        match self {
            Self::Get(cmd) => cmd.exec().await,
        }
    }
}
