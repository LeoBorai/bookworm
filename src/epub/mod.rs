mod container;
mod toc;

pub use container::{MetaInfContainer, RootFile};
pub use toc::{Toc, TocMeta};

use std::fs::File;
use std::path::Path;

use anyhow::Result;
use tokio::sync::Mutex;
use zip::ZipArchive;

use crate::epub::container::CONTAINER_XML;
use crate::epub::toc::TOC_NCX;
use crate::util::zip::get_file_bytes;

#[derive(Debug)]
pub struct Epub {
    archive: Mutex<ZipArchive<File>>,
    mic: MetaInfContainer,
    toc: Toc,
}

impl Epub {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Epub> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let container_xml = get_file_bytes(&mut archive, CONTAINER_XML)?;
        let mic = MetaInfContainer::new(container_xml)?;
        let toc_ncx = get_file_bytes(&mut archive, TOC_NCX)?;
        let toc = Toc::new(toc_ncx)?;

        Ok(Epub {
            archive: Mutex::new(archive),
            mic,
            toc,
        })
    }

    /// Returns the `dtb:uid` from the `toc.ncx` file, which is typically the ISBN of the EPUB.
    pub fn isbn(&self) -> &String {
        &self.toc.meta.uid
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
