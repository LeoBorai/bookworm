mod get;
mod set;

use anyhow::Result;
use clap::Subcommand;

use crate::cmd::pdf::meta::get::GetOpt;
use crate::cmd::pdf::meta::set::SetOpt;

#[derive(Clone, Debug, Subcommand)]
pub enum MetadataCmd {
    /// Retrieve PDF file metadata
    Get(GetOpt),
    /// Update PDF file metadata
    Set(SetOpt),
}

impl MetadataCmd {
    pub async fn exec(&self) -> Result<()> {
        match self {
            Self::Get(cmd) => cmd.exec().await,
            Self::Set(cmd) => cmd.exec().await,
        }
    }
}
