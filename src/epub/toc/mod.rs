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
