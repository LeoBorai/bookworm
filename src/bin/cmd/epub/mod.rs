mod meta;
mod unpackage;

use anyhow::Result;
use clap::Subcommand;

use self::meta::MetadataCmd;
use self::unpackage::UnPackageOpt;

#[derive(Clone, Debug, Subcommand)]
pub enum EpubCmd {
    /// Retrieve EPUB file metadata
    #[clap(subcommand)]
    Meta(MetadataCmd),
    /// Unpackage EPUB file into a directory
    Unpkg(UnPackageOpt),
}

impl EpubCmd {
    pub async fn exec(&self) -> Result<()> {
        match self {
            Self::Meta(cmd) => cmd.exec().await,
            Self::Unpkg(cmd) => cmd.exec().await,
        }
    }
}
