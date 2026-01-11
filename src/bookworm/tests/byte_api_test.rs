use std::fs;
use std::io::Cursor;

use anyhow::Result;
use bookworm::pdf::Pdf;

#[test]
fn test_pdf_from_bytes() -> Result<()> {
    // Read a PDF file into memory
    let pdf_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/Adobe PDF Manual.pdf"
    );
    let bytes = fs::read(pdf_path)?;

    // Create a Pdf from bytes
    let pdf = Pdf::from_bytes(&bytes)?;

    // Verify we can read metadata
    let version = pdf.version();
    assert!(!version.is_empty());

    Ok(())
}

#[test]
fn test_pdf_from_reader() -> Result<()> {
    // Read a PDF file into memory
    let pdf_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/Adobe PDF Manual.pdf"
    );
    let bytes = fs::read(pdf_path)?;

    // Create a Pdf from a reader (Cursor in this case)
    let cursor = Cursor::new(bytes);
    let pdf = Pdf::from_reader(cursor)?;

    // Verify we can read metadata
    let version = pdf.version();
    assert!(!version.is_empty());

    Ok(())
}

#[test]
fn test_pdf_save_to_bytes() -> Result<()> {
    // Read a PDF file into memory
    let pdf_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/Adobe PDF Manual.pdf"
    );
    let bytes = fs::read(pdf_path)?;

    // Create a Pdf from bytes
    let mut pdf = Pdf::from_bytes(&bytes)?;

    // Save to bytes
    let output_bytes = pdf.save_to_bytes()?;

    // Verify output has content
    assert!(!output_bytes.is_empty());

    // Verify we can load it again
    let pdf2 = Pdf::from_bytes(&output_bytes)?;
    assert_eq!(pdf.version(), pdf2.version());

    Ok(())
}
