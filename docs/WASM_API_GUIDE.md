# WASM/Browser API Guide

This guide explains how to use the byte-based APIs in bookworm for WASM32/browser environments.

## Overview

As of version 1.0.0-pre, bookworm supports byte-based APIs that don't require file system access. This makes the library fully compatible with WASM32 and browser environments.

## API Changes

### EPUB API

#### Reading EPUBs

**File-based (original API - still supported):**
```rust
use bookworm::epub::Epub;

// From file path
let epub = Epub::open("book.epub")?;
```

**Byte-based (new API for WASM/browser):**
```rust
use bookworm::epub::Epub;

// From byte vector
let bytes = fetch_epub_bytes(); // e.g., from fetch() in browser
let epub = Epub::from_bytes(bytes)?;

// From any reader (File, Cursor, etc.)
use std::io::Cursor;
let cursor = Cursor::new(bytes);
let epub = Epub::from_reader(cursor)?;
```

#### Writing EPUBs

**File-based (original API - still supported):**
```rust
use bookworm::epub::EpubWriter;
use std::fs::File;

async fn create_epub() -> anyhow::Result<()> {
    let file = File::create("output.epub")?;
    let mut writer = EpubWriter::new(file, "source_directory")?;
    writer.write().await?;
    Ok(())
}
```

**Byte-based (new API for WASM/browser):**
```rust
use bookworm::epub::EpubWriter;

async fn create_epub_in_memory() -> anyhow::Result<Vec<u8>> {
    // Write to in-memory buffer
    let mut writer = EpubWriter::new_in_memory("source_directory")?;
    writer.write().await?;
    let epub_bytes = writer.into_bytes()?;
    
    // Now you can send epub_bytes over network, save to IndexedDB, etc.
    Ok(epub_bytes)
}
```

### PDF API

#### Reading PDFs

**File-based (original API - still supported):**
```rust
use bookworm::pdf::Pdf;

// From file path
let pdf = Pdf::open("document.pdf")?;
```

**Byte-based (new API for WASM/browser):**
```rust
use bookworm::pdf::Pdf;

// From byte slice
let bytes = fetch_pdf_bytes(); // e.g., from fetch() in browser
let pdf = Pdf::from_bytes(&bytes)?;

// From any reader (File, Cursor, etc.)
use std::io::Cursor;
let cursor = Cursor::new(bytes);
let pdf = Pdf::from_reader(cursor)?;
```

#### Writing PDFs

**File-based (original API - still supported):**
```rust
let mut pdf = Pdf::open("document.pdf")?;
pdf.set_metadata(&PdfMetaField::Title, "New Title")?;
pdf.save("output.pdf")?;
```

**Byte-based (new API for WASM/browser):**
```rust
let mut pdf = Pdf::from_bytes(&bytes)?;
pdf.set_metadata(&PdfMetaField::Title, "New Title")?;
let output_bytes = pdf.save_to_bytes()?;

// Now you can send output_bytes over network, save to IndexedDB, etc.
```

## Browser Integration Example

Here's a complete example of how to use bookworm in a WASM application:

```rust
use wasm_bindgen::prelude::*;
use bookworm::pdf::Pdf;

#[wasm_bindgen]
pub fn process_pdf(bytes: Vec<u8>) -> Result<JsValue, JsValue> {
    // Create PDF from bytes
    let pdf = Pdf::from_bytes(&bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to load PDF: {}", e)))?;
    
    // Read metadata
    let metadata = pdf.metadata()
        .map_err(|e| JsValue::from_str(&format!("Failed to read metadata: {}", e)))?;
    
    // Return metadata as JSON
    let result = serde_json::json!({
        "version": pdf.version(),
        "title": metadata.title,
        "author": metadata.author,
    });
    
    Ok(JsValue::from_str(&result.to_string()))
}

#[wasm_bindgen]
pub fn modify_pdf_metadata(bytes: Vec<u8>, title: String) -> Result<Vec<u8>, JsValue> {
    // Create PDF from bytes
    let mut pdf = Pdf::from_bytes(&bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to load PDF: {}", e)))?;
    
    // Modify metadata
    pdf.set_metadata(&bookworm::pdf::PdfMetaField::Title, &title)
        .map_err(|e| JsValue::from_str(&format!("Failed to set metadata: {}", e)))?;
    
    // Return modified PDF as bytes
    pdf.save_to_bytes()
        .map_err(|e| JsValue::from_str(&format!("Failed to save PDF: {}", e)))
}
```

## Backward Compatibility

All original file-based APIs remain unchanged and fully supported. This means:

- Existing code using `Epub::open()` and `Pdf::open()` will continue to work without modifications
- File-based operations are still the recommended approach for desktop/server applications
- You only need to use the new byte-based APIs when targeting WASM/browser environments

## Benefits

The byte-based APIs provide several advantages:

1. **WASM Compatibility**: Works in browser environments without file system access
2. **Flexibility**: Can work with data from any source (network, IndexedDB, etc.)
3. **Memory Efficiency**: Can process data without intermediate file I/O
4. **Testing**: Easier to write unit tests with in-memory data
5. **Portability**: Same code works across platforms (desktop, server, browser)

## Type Safety

The API uses Rust's type system to ensure safety:

- `Epub<File>` for file-based EPUBs
- `Epub<Cursor<Vec<u8>>>` for byte-based EPUBs
- All types implement the same trait methods, ensuring consistent behavior

## Example Projects

See the `examples/byte_based_api.rs` file for a complete working example demonstrating:
- Loading PDFs from bytes
- Reading metadata without file system access
- Saving PDFs back to bytes
