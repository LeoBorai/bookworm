use std::path::Path;
use std::str::FromStr;
use std::{fs::File, io::Read};

use anyhow::{Context, Result, bail};
use lopdf::{Document, Object};
use memmap2::Mmap;

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
    /// Opens a PDF file from a file path.
    ///
    /// This is a convenience method for file-based operations.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let mut doc = Document::load_mem(&mmap)?;
        Self::decrypt_if_needed(&mut doc)?;
        Ok(Pdf { doc })
    }

    /// Creates a Pdf from a byte slice.
    ///
    /// This is useful for WASM/browser environments where file system access is not available.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut doc = Document::load_mem(bytes)?;
        Self::decrypt_if_needed(&mut doc)?;
        Ok(Pdf { doc })
    }

    /// Creates a Pdf from a generic reader.
    ///
    /// This is useful when you have a custom source that implements Read.
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let mut doc = Document::load_from(reader)?;
        Self::decrypt_if_needed(&mut doc)?;
        Ok(Pdf { doc })
    }

    /// Helper function to decrypt a PDF document if it's encrypted.
    fn decrypt_if_needed(doc: &mut Document) -> Result<()> {
        // https://github.com/J-F-Liu/lopdf/issues/453#issuecomment-3611121319
        if doc.is_encrypted() {
            match doc.decrypt("") {
                Ok(_) => {
                    doc.trailer.remove(b"Encrypt");
                }
                Err(e) => {
                    eprintln!("Failed to decrypt. {e:?}");
                }
            }
        }
        Ok(())
    }

    pub fn version(&self) -> &String {
        &self.doc.version
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

        bail!("Info dictionary not found in PDF document");
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.doc
            .save(&path)
            .context("Failed to save PDF document")?;
        Ok(())
    }

    /// Saves the PDF document to a byte vector.
    ///
    /// This is useful for WASM/browser environments where file system access is not available.
    pub fn save_to_bytes(&mut self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.doc
            .save_to(&mut buffer)
            .context("Failed to save PDF document to bytes")?;
        Ok(buffer)
    }
}
