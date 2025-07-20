mod container;

pub use container::{MetaInfContainer, RootFile};

use std::fs::File;
use std::path::Path;

use anyhow::Result;
use tokio::sync::Mutex;
use zip::ZipArchive;

use crate::{epub::container::CONTAINER_XML, util::zip::get_file_bytes};

#[derive(Debug)]
pub struct Epub {
    archive: Mutex<ZipArchive<File>>,
    mic: MetaInfContainer,
}

impl Epub {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Epub> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let container_xml = get_file_bytes(&mut archive, CONTAINER_XML)?;
        let mic = MetaInfContainer::new(container_xml)?;

        Ok(Epub {
            archive: Mutex::new(archive),
            mic,
        })
    }

    // pub async fn check(&self) -> Result<()> {
    //     let mut archive = self.archive.lock().await;
    //     let xml_reader = EventReader::new(archive.by_name(TOC_NCX)?);

    //     for event in xml_reader {
    //         println!("{event:?}");
    //         // match event? {
    //         //     XmlEvent::StartElement { name, .. } => {
    //         //         if name.local_name == "container" {
    //         //             return Ok(());
    //         //         }
    //         //     }
    //         //     XmlEvent::EndElement { name } => {
    //         //         if name.local_name == "container" {
    //         //             return Ok(());
    //         //         }
    //         //     }
    //         //     _ => {}
    //         // }
    //     }

    //     Err(anyhow::anyhow!("Invalid EPUB format: missing container element"))
    // }
}
