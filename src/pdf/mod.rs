use std::path::Path;

use anyhow::Result;
use lopdf::Document;

const PDF_META_INFO_KEY: &[u8] = b"Info";
const PDF_META_TITLE_KEY: &[u8] = b"Title";
const PDF_META_AUTHOR_KEY: &[u8] = b"Author";
const PDF_META_CREATOR_KEY: &[u8] = b"Creator";
const PDF_META_PRODUCER_KEY: &[u8] = b"Producer";
const PDF_META_CREATION_DATE_KEY: &[u8] = b"CreationDate";
const PDF_META_MODIFICATION_DATE_KEY: &[u8] = b"ModDate";

#[derive(Debug)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
}

#[derive(Debug)]
pub struct Pdf {
    doc: Document,
}

impl Pdf {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let doc = Document::load(path)?;
        Ok(Pdf { doc })
    }

    pub fn metadata(&self) -> Result<PdfMetadata> {
        Ok(PdfMetadata {
            title: self.get_metadata_field(PDF_META_TITLE_KEY),
            author: self.get_metadata_field(PDF_META_AUTHOR_KEY),
            creator: self.get_metadata_field(PDF_META_CREATOR_KEY),
            producer: self.get_metadata_field(PDF_META_PRODUCER_KEY),
            creation_date: self.get_metadata_field(PDF_META_CREATION_DATE_KEY),
            modification_date: self.get_metadata_field(PDF_META_MODIFICATION_DATE_KEY),
        })
    }

    fn get_metadata_field(&self, field: &[u8]) -> Option<String> {
        let doc = &self.doc;
        let info_ref = doc.trailer.get(PDF_META_INFO_KEY).ok()?;
        let object_id = match info_ref {
            lopdf::Object::Reference(id) => *id,
            _ => return None,
        };

        if !doc.objects.contains_key(&object_id) {
            return None;
        }

        let info_obj = doc.get_object(object_id).ok()?;
        let dict = info_obj.as_dict().ok()?;

        dict.get(field)
            .ok()
            .and_then(|value| value.as_str().ok())
            .map(|bytes| String::from_utf8_lossy(bytes).to_string())
    }
}
