mod info;

use clap::Subcommand;

use self::info::InfoOpt;

#[derive(Clone, Debug, Subcommand)]
pub enum EpubCmd {
    /// Retrieve (K)Epub File Information
    Info(InfoOpt),
}

impl EpubCmd {
    pub async fn exec(&self) -> anyhow::Result<()> {
        match self {
            Self::Info(cmd) => cmd.exec().await,
        }
    }
}
