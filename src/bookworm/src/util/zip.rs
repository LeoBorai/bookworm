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

    // Pre-allocate buffer based on file size to avoid incremental growing
    let mut buffer = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}
