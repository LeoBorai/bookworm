mod info;

use anyhow::Result;
use clap::Subcommand;

use self::info::InfoOpt;

#[derive(Clone, Debug, Subcommand)]
pub enum PdfCmd {
    /// Retrieve PDF File Information
    Info(InfoOpt),
}

impl PdfCmd {
    pub async fn exec(&self) -> Result<()> {
        match self {
            Self::Info(cmd) => cmd.exec().await,
        }
    }
}
