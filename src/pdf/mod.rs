use std::{path::Path, str::FromStr};

use anyhow::Result;
use lopdf::{Document, Object};

const PDF_META_INFO_KEY: &[u8] = b"Info";
const PDF_META_TITLE_KEY: &[u8] = b"Title";
const PDF_META_AUTHOR_KEY: &[u8] = b"Author";
const PDF_META_CREATOR_KEY: &[u8] = b"Creator";
const PDF_META_PRODUCER_KEY: &[u8] = b"Producer";
const PDF_META_CREATION_DATE_KEY: &[u8] = b"CreationDate";
const PDF_META_MODIFICATION_DATE_KEY: &[u8] = b"ModDate";

#[derive(Debug, Clone, Copy)]
pub enum PdfMetaField {
    Title,
    Author,
    Creator,
    Producer,
    CreationDate,
    ModificationDate,
}

impl FromStr for PdfMetaField {
    type Err = ();

    fn from_str(input: &str) -> std::result::Result<PdfMetaField, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "title" => Ok(PdfMetaField::Title),
            "author" => Ok(PdfMetaField::Author),
            "creator" => Ok(PdfMetaField::Creator),
            "producer" => Ok(PdfMetaField::Producer),
            "creationdate" => Ok(PdfMetaField::CreationDate),
            "modificationdate" => Ok(PdfMetaField::ModificationDate),
            _ => Err(()),
        }
    }
}

impl PdfMetaField {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            PdfMetaField::Title => PDF_META_TITLE_KEY,
            PdfMetaField::Author => PDF_META_AUTHOR_KEY,
            PdfMetaField::Creator => PDF_META_CREATOR_KEY,
            PdfMetaField::Producer => PDF_META_PRODUCER_KEY,
            PdfMetaField::CreationDate => PDF_META_CREATION_DATE_KEY,
            PdfMetaField::ModificationDate => PDF_META_MODIFICATION_DATE_KEY,
        }
    }
}

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
        let doc = Document::load(&path)?;

        Ok(Pdf { doc })
    }

    pub fn metadata(&self) -> Result<PdfMetadata> {
        Ok(PdfMetadata {
            title: self.get_metadata_field(&PdfMetaField::Title),
            author: self.get_metadata_field(&PdfMetaField::Author),
            creator: self.get_metadata_field(&PdfMetaField::Creator),
            producer: self.get_metadata_field(&PdfMetaField::Producer),
            creation_date: self.get_metadata_field(&PdfMetaField::CreationDate),
            modification_date: self.get_metadata_field(&PdfMetaField::ModificationDate),
        })
    }

    fn get_metadata_field(&self, field: &PdfMetaField) -> Option<String> {
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

        dict.get(field.as_bytes())
            .ok()
            .and_then(|value| value.as_str().ok())
            .map(|bytes| String::from_utf8_lossy(bytes).to_string())
    }

    pub fn set_metadata(&mut self, field: &PdfMetaField, value: &str) -> Result<()> {
        if let Ok(info) = self.doc.trailer.get_mut(PDF_META_INFO_KEY)
            && let Some(dict) = match info {
                Object::Dictionary(dict) => Some(dict),
                Object::Reference(id) => self
                    .doc
                    .objects
                    .get_mut(id)
                    .and_then(|o| o.as_dict_mut().ok()),
                _ => None,
            }
        {
            dict.set(field.as_bytes(), Object::string_literal(value));
            return Ok(());
        }

        anyhow::bail!("Info dictionary not found in PDF document");
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.doc.save(&path)
    }
}
