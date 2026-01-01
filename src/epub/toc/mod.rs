mod doc_title;
mod toc_meta;

use std::fs::File;

use anyhow::{Result, bail};
use zip::ZipArchive;

pub use self::doc_title::DocTitle;
pub use self::toc_meta::TocMeta;

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

    pub fn resolve_toc_ncx_file(zip: &mut ZipArchive<File>) -> Result<String> {
        const TOP_LEVEL_TOC_PATH: &str = "toc.ncx";
        const DEFAULT_TOC_PATH: &str = "OEBPS/toc.ncx";
        const ALTERNATIVE_TOC_PATH: &str = "OPS/toc.ncx";
        const HTML_DIR_TOC_PATH: &str = "OEBPS/html/toc.ncx";

        if zip.by_name(DEFAULT_TOC_PATH).is_ok() {
            return Ok(DEFAULT_TOC_PATH.to_string());
        }

        if zip.by_name(ALTERNATIVE_TOC_PATH).is_ok() {
            return Ok(ALTERNATIVE_TOC_PATH.to_string());
        }

        if zip.by_name(HTML_DIR_TOC_PATH).is_ok() {
            return Ok(HTML_DIR_TOC_PATH.to_string());
        }

        if zip.by_name(TOP_LEVEL_TOC_PATH).is_ok() {
            return Ok(TOP_LEVEL_TOC_PATH.to_string());
        }

        bail!("Failed to resolve TOC file path")
    }
}

// Enable tests when fixtures with Creative Commons license are available
// #[cfg(test)]
// mod test {
//     use super::*;

//     const TOC_NCX_BYTES: &[u8] = include_bytes!("../../../fixtures/toc.ncx");

//     #[tokio::test]
//     async fn parses_toc_ncx() -> Result<()> {
//         let toc = Toc::new(TOC_NCX_BYTES.to_vec())?;

//         assert_eq!(toc.meta.uid, "9781098166304");
//         assert_eq!(toc.doc_title.title, "AI Engineering");

//         Ok(())
//     }
// }
