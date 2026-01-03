mod container;
mod content_opf;
mod toc;
mod writer;

pub use container::{MetaInfContainer, RootFile};
pub use toc::{Toc, TocMeta};
pub use writer::EpubWriter;

use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::sync::Mutex;
use zip::ZipArchive;

use crate::epub::container::CONTAINER_XML;
use crate::epub::content_opf::ContentOpf;
use crate::util::zip::get_file_bytes;

/// Represents an EPUB file and provides access to its components.
///
/// ## EPUB File Structure
///
/// ```ignore
/// book.epub (ZIP archive)
/// ├── mimetype                          # Must be FIRST, UNCOMPRESSED
/// ├── META-INF/
/// │   ├── container.xml                 # Points to OPF file location
/// │   ├── encryption.xml                # (optional, DRM)
/// │   └── rights.xml                    # (optional, DRM)
/// ├── OEBPS/ (or EPUB/ or OPS/)        # Content directory (name varies)
/// │   ├── content.opf                   # Package document (metadata, manifest, spine)
/// │   ├── toc.ncx                       # Navigation (EPUB2) or nav.xhtml (EPUB3)
/// │   ├── Text/                         # XHTML content files
/// │   │   ├── chapter01.xhtml
/// │   │   ├── chapter02.xhtml
/// │   │   └── ...
/// │   ├── Styles/
/// │   │   └── stylesheet.css
/// │   ├── Images/
/// │   │   ├── cover.jpg
/// │   │   └── ...
/// │   └── Fonts/
/// │       └── font.ttf
/// └── ...
/// ```
#[derive(Debug)]
pub struct Epub {
    #[allow(unused)]
    archive: Mutex<ZipArchive<File>>,
    mic: MetaInfContainer,
    toc: Toc,
    content_opf: ContentOpf,
}

impl Epub {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Epub> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let container_xml = get_file_bytes(&mut archive, CONTAINER_XML)?;
        let mic = MetaInfContainer::new(container_xml)?;
        let toc_ncx_path = Toc::resolve_toc_ncx_file(&mut archive)?;
        let toc_ncx = get_file_bytes(&mut archive, &toc_ncx_path)?;
        let toc = Toc::new(toc_ncx)?;
        let opf_path = ContentOpf::resolve_opf_file(&mut archive, &mic)?;
        let content_opf_bytes = get_file_bytes(&mut archive, &opf_path)?;
        let content_opf = ContentOpf::new(content_opf_bytes)?;

        Ok(Epub {
            archive: Mutex::new(archive),
            mic,
            toc,
            content_opf,
        })
    }

    pub fn unpackage<P: AsRef<Path>>(path: P, outdir: P) -> Result<PathBuf> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        archive.extract(&outdir)?;
        Ok(outdir.as_ref().to_path_buf())
    }

    /// Returns the `dtb:uid` from the `toc.ncx` file, which is typically the ISBN of the EPUB.
    pub fn isbn(&self) -> &String {
        &self.toc.meta.uid
    }

    pub fn toc(&self) -> &Toc {
        &self.toc
    }

    pub fn mic(&self) -> &MetaInfContainer {
        &self.mic
    }

    pub fn content_opf(&self) -> &ContentOpf {
        &self.content_opf
    }
}
