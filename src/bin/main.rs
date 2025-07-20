use anyhow::Result;

use kepub::epub::Epub;

#[tokio::main]
async fn main() -> Result<()> {
    let epub = Epub::open("fixtures/file.epub")?;
    let isbn = epub.isbn();

    Ok(())
}
