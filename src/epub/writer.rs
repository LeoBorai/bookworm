use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use zip::write::{ExtendedFileOptions, FileOptions, ZipWriter};

use crate::epub::Epub;

pub struct EpubWriter {
    source: PathBuf,
    zip_writer: ZipWriter<File>,
}

impl EpubWriter {
    pub fn new<P: AsRef<Path>>(file: File, source: P) -> Result<Self> {
        let source = PathBuf::from(source.as_ref());
        let zip_writer = ZipWriter::new(file);

        if !source.is_dir() {
            bail!("The source '{:?}' does't belongs to a directory", source)
        }

        Ok(EpubWriter { source, zip_writer })
    }

    pub async fn write(&mut self) -> Result<()> {
        self.write_mimetype()?;
        self.write_meta_inf().await?;
        Ok(())
    }

    /// Writes the `mimetype` file as the first file in the EPUB archive.
    fn write_mimetype(&mut self) -> Result<()> {
        let options: FileOptions<'_, ExtendedFileOptions> =
            FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        self.zip_writer.start_file("mimetype", options)?;
        self.zip_writer.write_all(b"application/epub+zip")?;
        Ok(())
    }

    /// Copies the contents of the META-INF directory into the EPUB archive.
    async fn write_meta_inf(&mut self) -> Result<()> {
        let path = self.source.join("META-INF");
        let file = File::open(&path)?;

        if !file.metadata()?.is_dir() {
            bail!("The source '{:?}' is not a directory", path)
        }

        let options: FileOptions<'_, ExtendedFileOptions> =
            FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let file_path = entry.path();
            let file_name = file_path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| anyhow::anyhow!("Failed to get file name"))?;
            let mut file = File::open(&file_path)?;
            let mut buffer = Vec::new();
            use std::io::Read;
            file.read_to_end(&mut buffer)?;

            self.zip_writer
                .start_file(format!("META-INF/{}", file_name), options.clone())?;
            self.zip_writer.write_all(&buffer)?;
        }

        Ok(())
    }
}
