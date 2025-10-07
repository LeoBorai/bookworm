use std::path::PathBuf;

use anyhow::Result;
use xml::{EventReader, reader::XmlEvent};

pub const CONTAINER_XML: &str = "META-INF/container.xml";

/// The `rootfile` element in the `META-INF/container.xml` file
#[derive(Debug, Clone)]
pub struct RootFile {
    pub full_path: PathBuf,
    pub media_type: String,
}

/// Representation of the `META-INF/container.xml` file in an EPUB archive.
#[derive(Debug)]
pub struct MetaInfContainer {
    pub rootfiles: Vec<RootFile>,
}

impl MetaInfContainer {
    /// Parses the `META-INF/container.xml` file and extracts the root files.
    pub fn new(container_xml: Vec<u8>) -> Result<Self> {
        let xml_str = String::from_utf8(container_xml)
            .map_err(|e| anyhow::anyhow!("Failed to convert bytes to string: {}", e))?;
        let xml_reader = EventReader::from_str(&xml_str);
        let mut rootfiles: Vec<RootFile> = Vec::new();

        for maybe_event in xml_reader {
            if let Ok(event) = maybe_event
                && let XmlEvent::StartElement {
                    name, attributes, ..
                } = event
                && name.local_name == "rootfile"
            {
                let media_type = attributes
                    .iter()
                    .find(|attr| attr.name.local_name == "media-type")
                    .map_or_else(
                        || "application/oebps-package+xml".to_string(),
                        |attr| attr.value.clone(),
                    );

                let full_path = attributes
                    .iter()
                    .find(|attr| attr.name.local_name == "full-path")
                    .map_or_else(|| PathBuf::from(""), |attr| PathBuf::from(&attr.value));

                rootfiles.push(RootFile {
                    media_type,
                    full_path: full_path.clone(),
                });
            }
        }

        Ok(Self { rootfiles })
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use anyhow::Result;

    use crate::epub::MetaInfContainer;

    const CONTAINER_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
    <rootfiles>
        <rootfile full-path="OEBPS/9781718500457.opf" media-type="application/oebps-package+xml" />
    </rootfiles>
</container>
"#;

    #[tokio::test]
    async fn parses_container_xml_accordingly() -> Result<()> {
        let container_xml_bytes = CONTAINER_XML.as_bytes().to_vec();
        let mic = MetaInfContainer::new(container_xml_bytes)?;

        assert_eq!(mic.rootfiles.len(), 1);
        assert_eq!(mic.rootfiles[0].media_type, "application/oebps-package+xml");
        assert_eq!(
            mic.rootfiles[0].full_path,
            PathBuf::from("OEBPS/9781718500457.opf")
        );

        Ok(())
    }
}
