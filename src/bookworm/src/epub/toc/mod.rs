mod doc_title;
mod toc_meta;

use std::io::{Read, Seek};

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
    /// Parses the `OEBPS/toc.ncx` file and extracts metadata and document title in a single pass.
    pub fn new(bytes: Vec<u8>) -> Result<Toc> {
        use std::io::Cursor;
        use xml::{EventReader, reader::XmlEvent};

        let cursor = Cursor::new(bytes);
        let xml_reader = EventReader::new(cursor);
        
        let mut uid = String::new();
        let mut title = String::new();
        let mut in_doc_title = false;

        // Parse XML, silently skipping errors (consistent with original implementation)
        for event in xml_reader.into_iter().flatten() {
            match event {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    if name.local_name == "meta" {
                        let name_attr = attributes
                            .iter()
                            .find(|attr| attr.name.local_name == "name");
                        let content_attr = attributes
                            .iter()
                            .find(|attr| attr.name.local_name == "content");

                        if let (Some(name), Some(content)) = (name_attr, content_attr)
                            && name.value == "dtb:uid"
                        {
                            uid = content.value.clone();
                        }
                    } else if name.local_name == "docTitle" {
                        in_doc_title = true;
                    }
                }
                XmlEvent::Characters(text) => {
                    if in_doc_title && !text.trim().is_empty() {
                        title = text;
                        in_doc_title = false;
                        // Continue parsing to get uid if we haven't found it yet
                        if !uid.is_empty() {
                            break; // We have both values, can exit early
                        }
                    }
                }
                XmlEvent::EndElement { name } => {
                    if name.local_name == "ncx" {
                        break; // End of the toc.ncx file
                    }
                }
                _ => {}
            }
        }

        Ok(Self {
            meta: TocMeta { uid },
            doc_title: DocTitle { title },
        })
    }

    pub fn resolve_toc_ncx_file<R: Read + Seek>(zip: &mut ZipArchive<R>) -> Result<String> {
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
