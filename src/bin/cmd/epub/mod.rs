mod info;
mod package;
mod unpackage;

use clap::Subcommand;

use self::info::InfoOpt;
use self::package::PackageOpt;
use self::unpackage::UnPackageOpt;

#[derive(Clone, Debug, Subcommand)]
pub enum EpubCmd {
    /// Retrieve (K)Epub File Information
    Info(InfoOpt),
    /// Package (K)Epub File
    Package(PackageOpt),
    /// Unpackage (K)Epub File
    Unpkg(UnPackageOpt),
}

impl EpubCmd {
    pub async fn exec(&self) -> anyhow::Result<()> {
        match self {
            Self::Info(cmd) => cmd.exec().await,
            Self::Package(cmd) => cmd.exec().await,
            Self::Unpkg(cmd) => cmd.exec().await,
        }
    }
}
