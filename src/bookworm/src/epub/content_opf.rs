use std::io::{Read, Seek};

use anyhow::{Result, bail};
use xml::reader::{EventReader, XmlEvent};
use zip::ZipArchive;

use crate::epub::MetaInfContainer;

#[derive(Debug, Default)]
pub struct Metadata {
    pub title: String,
    pub creator: String,
    pub language: String,
    pub identifier: String,
}

#[derive(Debug)]
pub struct ManifestItem {
    pub id: String,
    pub href: String,
    pub media_type: String,
}

#[derive(Debug)]
pub struct SpineItem {
    pub idref: String,
}

/// ContentOpf represents the `content.opf` file in an EPUB archive.
/// It can either be a file in the path: `OEBPS/content.opf` or
/// `OEBPS/{ISBN}.opf`
#[derive(Debug)]
pub struct ContentOpf {
    pub metadata: Metadata,
    pub manifest: Vec<ManifestItem>,
    pub spine: Vec<SpineItem>,
}

impl ContentOpf {
    pub fn new(bytes: Vec<u8>) -> Result<ContentOpf> {
        // Parse XML directly from bytes using a Cursor instead of converting to String first
        let cursor = std::io::Cursor::new(bytes);
        let xml_reader = EventReader::new(cursor);

        let mut content_opf = ContentOpf {
            metadata: Metadata::default(),
            manifest: Vec::new(),
            spine: Vec::new(),
        };

        let mut current_element = String::new();
        let mut in_metadata = false;
        let mut in_manifest = false;
        let mut in_spine = false;

        for event in xml_reader {
            match event? {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    let element_name = name.local_name;

                    match element_name.as_str() {
                        "metadata" => in_metadata = true,
                        "manifest" => in_manifest = true,
                        "spine" => in_spine = true,
                        "item" if in_manifest => {
                            let mut item = ManifestItem {
                                id: String::new(),
                                href: String::new(),
                                media_type: String::new(),
                            };

                            for attr in attributes {
                                match attr.name.local_name.as_str() {
                                    "id" => item.id = attr.value,
                                    "href" => item.href = attr.value,
                                    "media-type" => item.media_type = attr.value,
                                    _ => {}
                                }
                            }

                            content_opf.manifest.push(item);
                        }
                        "itemref" if in_spine => {
                            for attr in attributes {
                                if attr.name.local_name == "idref" {
                                    content_opf.spine.push(SpineItem { idref: attr.value });
                                }
                            }
                        }
                        _ => {
                            current_element = element_name;
                        }
                    }
                }
                XmlEvent::EndElement { name } => match name.local_name.as_str() {
                    "metadata" => in_metadata = false,
                    "manifest" => in_manifest = false,
                    "spine" => in_spine = false,
                    _ => {}
                },
                XmlEvent::Characters(text) => {
                    if in_metadata {
                        match current_element.as_str() {
                            "title" => content_opf.metadata.title = text,
                            "creator" => content_opf.metadata.creator = text,
                            "language" => content_opf.metadata.language = text,
                            "identifier" => content_opf.metadata.identifier = text,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(content_opf)
    }

    pub fn resolve_opf_file<R: Read + Seek>(
        zip: &mut ZipArchive<R>,
        mic: &MetaInfContainer,
    ) -> Result<String> {
        const TOP_LEVEL_OPF_PATH: &str = "content.opf";
        const DEFAULT_OPF_PATH: &str = "OEBPS/content.opf";
        const ALTERNATIVE_OPF_PATH: &str = "OPS/content.opf";

        let opf_path = mic.rootfiles[0].full_path.to_str();

        // Check all possible paths in order, returning the first valid one
        let paths_to_check = [
            opf_path.filter(|p| p.ends_with("opf")),
            Some(DEFAULT_OPF_PATH),
            Some(ALTERNATIVE_OPF_PATH),
            Some(TOP_LEVEL_OPF_PATH),
        ];

        for path in paths_to_check.iter().flatten() {
            if zip.by_name(path).is_ok() {
                return Ok((*path).to_string());
            }
        }

        bail!("Failed to resolve OPF file path")
    }
}
