mod info;
mod unpackage;

use clap::Subcommand;

use self::info::InfoOpt;
use self::unpackage::UnPackageOpt;

#[derive(Clone, Debug, Subcommand)]
pub enum EpubCmd {
    /// Retrieve (K)Epub File Information
    Info(InfoOpt),
    /// Unpackage (K)Epub File
    Unpkg(UnPackageOpt),
}

impl EpubCmd {
    pub async fn exec(&self) -> anyhow::Result<()> {
        match self {
            Self::Info(cmd) => cmd.exec().await,
            Self::Unpkg(cmd) => cmd.exec().await,
        }
    }
}
