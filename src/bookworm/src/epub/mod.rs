mod container;
mod content_opf;
mod toc;
mod writer;

pub use container::{MetaInfContainer, RootFile};
pub use toc::{Toc, TocMeta};
pub use writer::EpubWriter;

use std::fs::File;
use std::io::{Cursor, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::Result;
use zip::ZipArchive;
use zip::write::{ExtendedFileOptions, FileOptions, ZipWriter};

use crate::epub::container::CONTAINER_XML;
use crate::epub::content_opf::ContentOpf;
use crate::util::zip::get_file_bytes;

/// Metadata fields that can be updated in an EPUB file.
#[derive(Debug, Clone, Copy)]
pub enum EpubMetaField {
    Title,
    Creator,
    Language,
    Identifier,
}

impl EpubMetaField {
    /// Returns the Dublin Core element local name used in OPF metadata.
    fn dc_element_name(&self) -> &'static str {
        match self {
            EpubMetaField::Title => "title",
            EpubMetaField::Creator => "creator",
            EpubMetaField::Language => "language",
            EpubMetaField::Identifier => "identifier",
        }
    }
}

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
pub struct Epub<R: Read + Seek> {
    archive: Mutex<ZipArchive<R>>,
    mic: MetaInfContainer,
    toc: Toc,
    content_opf: ContentOpf,
    content_opf_path: String,
    content_opf_raw: Vec<u8>,
}

impl<R: Read + Seek> Epub<R> {
    /// Creates an Epub from a generic reader (e.g., File, Cursor<Vec<u8>>).
    pub fn from_reader(reader: R) -> Result<Epub<R>> {
        let mut archive = ZipArchive::new(reader)?;
        let container_xml = get_file_bytes(&mut archive, CONTAINER_XML)?;
        let mic = MetaInfContainer::new(container_xml)?;
        let toc_ncx_path = Toc::resolve_toc_ncx_file(&mut archive)?;
        let toc_ncx = get_file_bytes(&mut archive, &toc_ncx_path)?;
        let toc = Toc::new(toc_ncx)?;
        let opf_path = ContentOpf::resolve_opf_file(&mut archive, &mic)?;
        let content_opf_raw = get_file_bytes(&mut archive, &opf_path)?;
        let content_opf = ContentOpf::new(content_opf_raw.clone())?;

        Ok(Epub {
            archive: Mutex::new(archive),
            mic,
            toc,
            content_opf,
            content_opf_path: opf_path,
            content_opf_raw,
        })
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

    /// Updates a metadata field in the EPUB.
    ///
    /// This modifies the in-memory representation. Call [`save`] or [`save_to_bytes`]
    /// to persist the changes.
    pub fn set_metadata(&mut self, field: &EpubMetaField, value: &str) -> Result<()> {
        match field {
            EpubMetaField::Title => self.content_opf.metadata.title = value.to_string(),
            EpubMetaField::Creator => self.content_opf.metadata.creator = value.to_string(),
            EpubMetaField::Language => self.content_opf.metadata.language = value.to_string(),
            EpubMetaField::Identifier => {
                self.content_opf.metadata.identifier = value.to_string()
            }
        }

        self.content_opf_raw =
            replace_dc_element_content(&self.content_opf_raw, field.dc_element_name(), value)?;

        Ok(())
    }

    /// Saves the EPUB to a byte vector.
    ///
    /// This is useful for WASM/browser environments where file system access is not available.
    pub fn save_to_bytes(&mut self) -> Result<Vec<u8>> {
        let cursor = Cursor::new(Vec::new());
        let written = self.write_to(cursor)?;
        Ok(written.into_inner())
    }

    /// Writes the EPUB (with any pending metadata changes) to the given writer.
    fn write_to<W: Write + Seek>(&mut self, writer: W) -> Result<W> {
        let mut archive = self.archive.lock().unwrap();
        let mut zip_writer = ZipWriter::new(writer);

        // mimetype must be the first entry and must be stored uncompressed
        zip_writer.start_file(
            "mimetype",
            FileOptions::<ExtendedFileOptions>::default()
                .compression_method(zip::CompressionMethod::Stored),
        )?;
        zip_writer.write_all(b"application/epub+zip")?;

        for i in 0..archive.len() {
            let (name, compression, is_dir, bytes) = {
                let mut file = archive.by_index(i)?;
                let name = file.name().to_string();
                let compression = file.compression();
                let is_dir = file.is_dir();
                let mut bytes = Vec::new();
                if !is_dir && name != "mimetype" {
                    file.read_to_end(&mut bytes)?;
                }
                (name, compression, is_dir, bytes)
            };

            if name == "mimetype" {
                continue;
            }

            if is_dir {
                let opts: FileOptions<'_, ExtendedFileOptions> =
                    FileOptions::default().compression_method(zip::CompressionMethod::Stored);
                zip_writer.add_directory(&name, opts)?;
                continue;
            }

            let opts: FileOptions<'_, ExtendedFileOptions> =
                FileOptions::default().compression_method(compression);

            if name == self.content_opf_path {
                zip_writer.start_file(&name, opts)?;
                zip_writer.write_all(&self.content_opf_raw)?;
            } else {
                zip_writer.start_file(&name, opts)?;
                zip_writer.write_all(&bytes)?;
            }
        }

        let writer = zip_writer.finish()?;
        Ok(writer)
    }
}

impl Epub<File> {
    /// Opens an EPUB file from a file path.
    ///
    /// This is a convenience method for file-based operations.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Epub<File>> {
        let file = File::open(path)?;
        Epub::from_reader(file)
    }

    pub fn unpackage<P: AsRef<Path>>(path: P, outdir: P) -> Result<PathBuf> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        archive.extract(&outdir)?;
        Ok(outdir.as_ref().to_path_buf())
    }

    /// Saves the EPUB document to a file path.
    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let file = File::create(path)?;
        self.write_to(file)?;
        Ok(())
    }
}

impl Epub<Cursor<Vec<u8>>> {
    /// Creates an Epub from a byte vector.
    ///
    /// This is useful for WASM/browser environments where file system access is not available.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Epub<Cursor<Vec<u8>>>> {
        let cursor = Cursor::new(bytes);
        Epub::from_reader(cursor)
    }
}

/// Replaces the text content of a Dublin Core XML element (e.g., `dc:title`) in an OPF byte
/// slice.
///
/// Handles elements with and without attributes in the opening tag. If the element is not found,
/// the original bytes are returned unchanged. Returns an error if the bytes are not valid UTF-8.
fn replace_dc_element_content(xml: &[u8], dc_element: &str, new_value: &str) -> Result<Vec<u8>> {
    let xml_str = std::str::from_utf8(xml)
        .map_err(|e| anyhow::anyhow!("OPF content is not valid UTF-8: {e}"))?;

    let open_tag = format!("<dc:{dc_element}");
    let close_tag = format!("</dc:{dc_element}>");

    if let Some(tag_start) = xml_str.find(&open_tag) {
        // Find the end of the opening tag (may have attributes)
        if let Some(content_start_offset) = xml_str[tag_start..].find('>') {
            let content_start = tag_start + content_start_offset + 1;
            if let Some(close_start_offset) = xml_str[content_start..].find(&close_tag) {
                let close_start = content_start + close_start_offset;
                let old_content_len = close_start - content_start;
                let capacity = xml_str.len() - old_content_len + new_value.len();
                let mut result = String::with_capacity(capacity);
                result.push_str(&xml_str[..content_start]);
                result.push_str(new_value);
                result.push_str(&xml_str[close_start..]);
                return Ok(result.into_bytes());
            }
        }
    }

    Ok(xml.to_vec())
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Write};

    use anyhow::Result;
    use zip::write::{ExtendedFileOptions, FileOptions, ZipWriter};

    use super::{Epub, EpubMetaField};

    /// Builds a minimal in-memory EPUB for testing.
    fn make_test_epub() -> Vec<u8> {
        let buf = Vec::new();
        let cursor = Cursor::new(buf);
        let mut zip = ZipWriter::new(cursor);

        let stored: FileOptions<'_, ExtendedFileOptions> =
            FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        let deflated: FileOptions<'_, ExtendedFileOptions> =
            FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // mimetype — must be first and uncompressed
        zip.start_file("mimetype", stored).unwrap();
        zip.write_all(b"application/epub+zip").unwrap();

        // META-INF/container.xml
        zip.start_file("META-INF/container.xml", deflated.clone())
            .unwrap();
        zip.write_all(
            br#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>"#,
        )
        .unwrap();

        // OEBPS/toc.ncx
        zip.start_file("OEBPS/toc.ncx", deflated.clone()).unwrap();
        zip.write_all(
            br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE ncx PUBLIC "-//NISO//DTD ncx 2005-1//EN" "http://www.daisy.org/z3986/2005/ncx-2005-1.dtd">
<ncx xmlns="http://www.daisy.org/z3986/2005/ncx/" version="2005-1">
  <head>
    <meta name="dtb:uid" content="test-isbn-123"/>
  </head>
  <docTitle><text>Test Book</text></docTitle>
  <navMap/>
</ncx>"#,
        )
        .unwrap();

        // OEBPS/content.opf
        zip.start_file("OEBPS/content.opf", deflated).unwrap();
        zip.write_all(
            br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" version="2.0">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:opf="http://www.idpf.org/2007/opf">
    <dc:title>Original Title</dc:title>
    <dc:creator opf:role="aut">Original Author</dc:creator>
    <dc:language>en</dc:language>
    <dc:identifier id="BookId">test-isbn-123</dc:identifier>
  </metadata>
  <manifest/>
  <spine/>
</package>"#,
        )
        .unwrap();

        zip.finish().unwrap().into_inner()
    }

    #[test]
    fn set_metadata_updates_in_memory_fields() -> Result<()> {
        let epub_bytes = make_test_epub();
        let mut epub = Epub::from_bytes(epub_bytes)?;

        epub.set_metadata(&EpubMetaField::Title, "New Title")?;
        epub.set_metadata(&EpubMetaField::Creator, "New Author")?;
        epub.set_metadata(&EpubMetaField::Language, "fr")?;
        epub.set_metadata(&EpubMetaField::Identifier, "new-id-456")?;

        assert_eq!(epub.content_opf().metadata.title, "New Title");
        assert_eq!(epub.content_opf().metadata.creator, "New Author");
        assert_eq!(epub.content_opf().metadata.language, "fr");
        assert_eq!(epub.content_opf().metadata.identifier, "new-id-456");

        Ok(())
    }

    #[test]
    fn save_to_bytes_round_trips_metadata() -> Result<()> {
        let epub_bytes = make_test_epub();
        let mut epub = Epub::from_bytes(epub_bytes)?;

        epub.set_metadata(&EpubMetaField::Title, "Round Trip Title")?;
        epub.set_metadata(&EpubMetaField::Creator, "Round Trip Author")?;

        let saved_bytes = epub.save_to_bytes()?;

        // Re-open the saved bytes and verify metadata persisted
        let epub2 = Epub::from_bytes(saved_bytes)?;
        assert_eq!(epub2.content_opf().metadata.title, "Round Trip Title");
        assert_eq!(epub2.content_opf().metadata.creator, "Round Trip Author");
        // Unchanged fields are preserved
        assert_eq!(epub2.content_opf().metadata.language, "en");

        Ok(())
    }
}
