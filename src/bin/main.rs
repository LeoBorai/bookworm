mod cmd;

use anyhow::Result;

use clap::Parser;

use crate::cmd::{convert::ConvertCmd, info::InfoCmd};

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
    name = "kepub",
    about = "Rakuten Kobo's k/epub utilities",
    author = "Leo Borai <estebanborai@gmail.com> (https://github.com/LeoBorai/kepub)",
    max_term_width = 100,
    next_line_help = true
)]
pub enum Command {
    /// Retrieve (K)Epub File Information
    Info(InfoCmd),
    /// Convert PDF to (K)Epub
    Convert(ConvertCmd),
}

impl Command {
    pub async fn exec(self) -> Result<()> {
        match self {
            Self::Info(cmd) => cmd.exec().await,
            Self::Convert(cmd) => cmd.exec().await,
        }
    }
}
