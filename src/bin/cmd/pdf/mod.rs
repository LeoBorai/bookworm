mod info;

use anyhow::Result;
use clap::Subcommand;

use self::info::InfoCmd;

#[derive(Clone, Debug, Subcommand)]
pub enum PdfCmd {
    #[clap(subcommand)]
    /// Retrieve PDF File Information
    Info(InfoCmd),
}

impl PdfCmd {
    pub async fn exec(&self) -> Result<()> {
        match self {
            Self::Info(cmd) => cmd.exec().await,
        }
    }
}
