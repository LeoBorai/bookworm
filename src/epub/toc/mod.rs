mod doc_title;
mod toc_meta;

use anyhow::Result;

pub use self::doc_title::DocTitle;
pub use self::toc_meta::TocMeta;

pub const TOC_NCX: &str = "OEBPS/toc.ncx";

/// `toc.ncx` file in an EPUB archive, which contains the table of contents.
#[derive(Debug, Clone)]
pub struct Toc {
    pub meta: TocMeta,
    pub doc_title: DocTitle,
}

impl Toc {
    /// Parses the `OEBPS/toc.ncx` file and extracts.
    pub fn new(bytes: Vec<u8>) -> Result<Toc> {
        let meta = TocMeta::try_from(bytes.clone())?;
        let doc_title = DocTitle::try_from(bytes.clone())?;

        Ok(Self { meta, doc_title })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TOC_NCX_BYTES: &[u8] = include_bytes!("../../../fixtures/toc.ncx");

    #[tokio::test]
    async fn parses_toc_ncx() -> Result<()> {
        let toc = Toc::new(TOC_NCX_BYTES.to_vec())?;

        assert_eq!(toc.meta.uid, "9781098166304");
        assert_eq!(toc.doc_title.title, "AI Engineering");

        Ok(())
    }
}
