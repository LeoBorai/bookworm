use std::io::Cursor;

use anyhow::Result;
use xml::{EventReader, reader::XmlEvent};

#[derive(Debug, Clone)]
pub struct TocMeta {
    /// The`dtb:uid` element
    pub uid: String,
}

impl TryFrom<Vec<u8>> for TocMeta {
    type Error = anyhow::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self> {
        let cursor = Cursor::new(bytes);
        let xml_reader = EventReader::new(cursor);
        let mut uid = String::new();

        for maybe_event in xml_reader {
            if let Ok(event) = maybe_event {
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
        }

        Ok(TocMeta { uid })
    }
}
