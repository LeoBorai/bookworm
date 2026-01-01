mod cmd;

use anyhow::Result;

use clap::Parser;

use crate::cmd::epub::EpubCmd;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.command.exec().await?;
    Ok(())
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
#[command(
    name = "bookworm",
    about = "Utilities to manage your ebook collection (PDFs, ePubs, KePubs, and more)",
    author = "Leo Borai <estebanborai@gmail.com> (https://github.com/LeoBorai/bookworm)",
    max_term_width = 100,
    next_line_help = true
)]
pub enum Command {
    /// Manage EPUB Files
    #[clap(subcommand)]
    Epub(EpubCmd),
}

impl Command {
    pub async fn exec(self) -> Result<()> {
        match self {
            Self::Epub(cmd) => cmd.exec().await,
        }
    }
}
