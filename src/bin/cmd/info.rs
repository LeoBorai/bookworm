use std::path::PathBuf;

use clap::Args;

use kepub::epub::Epub;

#[derive(Args, Clone, Debug)]
pub struct InfoCmd {
    /// Path to the (K)Epub file
    path: PathBuf,
}

impl InfoCmd {
    pub async fn exec(&self) -> anyhow::Result<()> {
        let epub = Epub::open(&self.path)?;
        let isbn = epub.isbn();
        let toc = epub.toc();

        println!("ISBN: {isbn:?}");
        println!("TOC: {toc:#?}");

        Ok(())
    }
}
