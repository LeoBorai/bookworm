use std::io::Cursor;

use anyhow::Result;
use xml::{EventReader, reader::XmlEvent};

#[derive(Debug, Clone)]
pub struct DocTitle {
    pub title: String,
}

impl TryFrom<Vec<u8>> for DocTitle {
    type Error = anyhow::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self> {
        let cursor = Cursor::new(bytes);
        let xml_reader = EventReader::new(cursor);
        let mut in_doc_title = false;
        let mut title = String::new();

        for event in xml_reader.into_iter().flatten() {
            match event {
                XmlEvent::StartElement { name, .. } => {
                    if name.local_name == "docTitle" {
                        in_doc_title = true;
                    }
                }
                XmlEvent::Characters(text) => {
                    if in_doc_title {
                        title = text;
                        break;
                    }
                }
                _ => {}
            }
        }

        Ok(Self { title })
    }
}
