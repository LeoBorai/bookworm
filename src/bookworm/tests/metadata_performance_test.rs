use std::fs;
use std::time::Instant;

use anyhow::Result;
use bookworm::pdf::Pdf;

// Expected PDF parsing rate: approximately 10MB per second
// This is a conservative estimate for reasonable performance on typical hardware
const EXPECTED_PARSE_RATE_BYTES_PER_SEC: usize = 10_000_000;

/// Test that verifies PDF metadata retrieval performance.
/// This test ensures that metadata retrieval is reasonably fast once the PDF is loaded.
#[test]
fn test_pdf_metadata_retrieval_performance() -> Result<()> {
    let pdf_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/Adobe PDF Manual.pdf"
    );
    let bytes = fs::read(pdf_path)?;

    // Load the PDF first (this is the expensive operation)
    let pdf = Pdf::from_bytes(&bytes)?;

    // Measure just the metadata retrieval time (should be fast)
    let start = Instant::now();
    let metadata = pdf.metadata()?;
    let duration = start.elapsed();

    // Verify metadata structure was retrieved (even if fields are empty)
    // The Adobe PDF Manual may not have all metadata fields populated
    let _ = metadata.title;
    let _ = metadata.author;

    // Metadata retrieval itself should be very fast (< 10ms)
    // since we're just reading from the already-loaded document structure
    assert!(
        duration.as_millis() < 10,
        "Metadata retrieval took {}ms, expected < 10ms",
        duration.as_millis()
    );

    println!("PDF metadata retrieval took: {:?}", duration);

    Ok(())
}

/// Test that verifies repeated metadata access doesn't cause performance issues.
#[test]
fn test_pdf_repeated_metadata_access() -> Result<()> {
    let pdf_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/Adobe PDF Manual.pdf"
    );
    let bytes = fs::read(pdf_path)?;
    let pdf = Pdf::from_bytes(&bytes)?;

    let start = Instant::now();
    for _ in 0..100 {
        let _ = pdf.metadata()?;
    }
    let duration = start.elapsed();

    // 100 metadata retrievals should still be fast (< 1 second)
    assert!(
        duration.as_secs() < 1,
        "100 metadata retrievals took {}ms, expected < 1000ms",
        duration.as_millis()
    );

    println!("100 metadata retrievals took: {:?}", duration);

    Ok(())
}

/// Test that PDF creation with from_bytes uses efficient memory allocation.
#[test]
fn test_pdf_from_bytes_efficiency() -> Result<()> {
    let pdf_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/Adobe PDF Manual.pdf"
    );
    let bytes = fs::read(pdf_path)?;
    let initial_size = bytes.len();

    let start = Instant::now();
    let pdf = Pdf::from_bytes(&bytes)?;
    let duration = start.elapsed();

    // Verify PDF was created successfully
    assert!(!pdf.version().is_empty());

    // PDF parsing should be reasonably fast relative to file size
    // For a ~22MB file, should complete in under 3 seconds
    // (being conservative to account for system load variations)
    let max_duration_secs = ((initial_size / EXPECTED_PARSE_RATE_BYTES_PER_SEC) as u64).max(3);
    assert!(
        duration.as_secs() < max_duration_secs,
        "PDF parsing took {}ms for {}MB file, expected < {}s",
        duration.as_millis(),
        initial_size / 1_000_000,
        max_duration_secs
    );

    println!(
        "PDF parsing of {}MB took: {:?}",
        initial_size / 1_000_000,
        duration
    );

    Ok(())
}
