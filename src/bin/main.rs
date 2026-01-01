mod cmd;

use anyhow::Result;

use clap::Parser;

use crate::cmd::info::InfoCmd;

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
    name = "bw",
    about = "Digital book utilities",
    author = "Leo Borai <estebanborai@gmail.com> (https://github.com/LeoBorai/bookworm)",
    max_term_width = 100,
    next_line_help = true
)]
pub enum Command {
    /// Retrieve (K)Epub File Information
    Info(InfoCmd),
}

impl Command {
    pub async fn exec(self) -> Result<()> {
        match self {
            Self::Info(cmd) => cmd.exec().await,
        }
    }
}
