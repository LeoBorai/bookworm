use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Result, bail};
use lopdf::{Dictionary, Document, Object};

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
        match input {
            "Title" => Ok(PdfMetaField::Title),
            "Author" => Ok(PdfMetaField::Author),
            "Creator" => Ok(PdfMetaField::Creator),
            "Producer" => Ok(PdfMetaField::Producer),
            "CreationDate" => Ok(PdfMetaField::CreationDate),
            "ModificationDate" => Ok(PdfMetaField::ModificationDate),
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
    path: PathBuf,
}

impl Pdf {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let doc = Document::load(&path)?;

        Ok(Pdf {
            doc,
            path: path.as_ref().to_path_buf(),
        })
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

    pub fn set_metadata(&self, field: &PdfMetaField, value: &str) -> Result<Pdf> {
        let mut doc = self.doc.clone();

        let info_id = if let Ok(info_ref) = doc.trailer.get(PDF_META_INFO_KEY) {
            match info_ref {
                Object::Reference(id) => *id,
                _ => Self::create_info_dictionary(&mut doc)?,
            }
        } else {
            Self::create_info_dictionary(&mut doc)?
        };

        if let Ok(info_obj) = doc.get_object_mut(info_id)
            && let Ok(dict) = info_obj.as_dict_mut()
        {
            dict.set(field.as_bytes(), Object::string_literal(value));
        }

        Ok(Self {
            doc,
            path: self.path.clone(),
        })
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        if self.path == path.as_ref() {
            bail!("Cannot overwrite source file");
        }

        self.doc.save(&path)?;
        Ok(())
    }

    fn create_info_dictionary(doc: &mut Document) -> Result<(u32, u16)> {
        let info_dict = Dictionary::new();
        let info_id = doc.add_object(Object::Dictionary(info_dict));
        doc.trailer
            .set(PDF_META_INFO_KEY, Object::Reference(info_id));
        Ok(info_id)
    }
}
