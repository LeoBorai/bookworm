mod get;
mod set;

use anyhow::Result;
use clap::Subcommand;

use crate::cmd::pdf::info::get::GetOpt;
use crate::cmd::pdf::info::set::SetOpt;

#[derive(Clone, Debug, Subcommand)]
pub enum InfoCmd {
    /// Retrieve PDF File Information
    Get(GetOpt),
    /// Set PDF File Information
    Set(SetOpt),
}

impl InfoCmd {
    pub async fn exec(&self) -> Result<()> {
        match self {
            Self::Get(cmd) => cmd.exec().await,
            Self::Set(cmd) => cmd.exec().await,
        }
    }
}
