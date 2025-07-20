use anyhow::Result;
use xml::{EventReader, reader::XmlEvent};

pub const TOC_NCX: &str = "OEBPS/toc.ncx";

#[derive(Debug, Clone)]
pub struct TocMeta {
    /// The`dtb:uid` element
    pub uid: String,
}

/// `toc.ncx` file in an EPUB archive, which contains the table of contents.
#[derive(Debug, Clone)]
pub struct Toc {
    pub meta: TocMeta,
}

impl Toc {
    /// Parses the `OEBPS/toc.ncx` file and extracts.
    pub fn new(toc_tcx: Vec<u8>) -> Result<Toc> {
        let xml_str = String::from_utf8(toc_tcx)
            .map_err(|e| anyhow::anyhow!("Failed to convert bytes to string: {}", e))?;
        let xml_reader = EventReader::from_str(&xml_str);
        let mut uid = String::new();

        for maybe_event in xml_reader {
            if let Ok(event) = maybe_event {
                match event {
                    XmlEvent::StartElement {
                        name, attributes, ..
                    } => {
                        // FIXME: There should be a better way to handle this.
                        if name.local_name == "meta" {
                            let name = attributes
                                .iter()
                                .find(|attr| attr.name.local_name == "name")
                                .map_or("", |attr| &attr.value);
                            let content = attributes
                                .iter()
                                .find(|attr| attr.name.local_name == "content")
                                .map_or("", |attr| &attr.value);

                            if name == "dtb:uid" {
                                uid = content.to_string();
                            }
                        }
                    }
                    XmlEvent::EndElement { name } => {
                        if name.local_name == "ncx" {
                            // End of the toc.ncx file
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Self {
            meta: TocMeta { uid },
        })
    }
}
