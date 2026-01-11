/// Example demonstrating byte-based PDF API for WASM/browser environments
///
/// This example shows how to use the new byte-based APIs that allow
/// bookworm to work in WASM32/browser environments where file system
/// access is not available.
use std::fs;

use anyhow::Result;
use bookworm::pdf::Pdf;

fn main() -> Result<()> {
    println!("=== Byte-based PDF API Example ===\n");

    // Step 1: Read a PDF file into memory
    let pdf_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/Adobe PDF Manual.pdf"
    );
    println!("Reading PDF from: {}", pdf_path);
    let bytes = fs::read(pdf_path)?;
    println!("Read {} bytes", bytes.len());

    // Step 2: Create a Pdf from bytes (no file system dependency!)
    println!("\nCreating PDF from bytes...");
    let pdf = Pdf::from_bytes(&bytes)?;
    println!("✓ Successfully created PDF from bytes");

    // Step 3: Read metadata
    println!("\nReading PDF metadata:");
    println!("  Version: {}", pdf.version());
    let metadata = pdf.metadata()?;
    println!(
        "  Title: {}",
        metadata.title.unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "  Author: {}",
        metadata.author.unwrap_or_else(|| "N/A".to_string())
    );

    // Step 4: Save to bytes (no file system dependency!)
    println!("\nSaving PDF to bytes...");
    let mut pdf_mut = Pdf::from_bytes(&bytes)?;
    let output_bytes = pdf_mut.save_to_bytes()?;
    println!(
        "✓ Successfully saved PDF to bytes ({} bytes)",
        output_bytes.len()
    );

    println!("\n=== Benefits for WASM/Browser ===");
    println!("• No file system access required");
    println!("• Works with JavaScript ArrayBuffer/Uint8Array");
    println!("• Perfect for browser-based PDF processing");
    println!("• Can be used with fetch() or FileReader API");

    Ok(())
}
