use std::io::{Read, Seek};

use anyhow::{Result, bail};
use zip::ZipArchive;

pub fn get_file_bytes<R: Read + Seek>(zip: &mut ZipArchive<R>, path: &str) -> Result<Vec<u8>> {
    let mut file = zip.by_name(path)?;

    if !file.is_file() {
        bail!(
            "Failed to get a file's bytes from EPUB. Path '{}' is not a file",
            path
        );
    }

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}
