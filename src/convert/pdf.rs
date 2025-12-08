use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::Utc;
use indicatif::{ProgressBar, ProgressStyle};
use lopdf::{Document, ObjectId};
use pdf_extract::extract_text;
use regex::Regex;

use crate::convert::{BookMetadata, Chapter};

pub struct PdfProcessor {
    chapter_regex: Regex,
    min_chapter_length: usize,
    max_pages_per_chapter: usize,
    clean_text: bool,
}

impl PdfProcessor {
    pub fn new(
        chapter_pattern: &str,
        min_chapter_length: usize,
        max_pages_per_chapter: usize,
        clean_text: bool,
    ) -> Result<Self> {
        let chapter_regex =
            Regex::new(chapter_pattern).context("Invalid chapter detection regex pattern")?;

        Ok(Self {
            chapter_regex,
            min_chapter_length,
            max_pages_per_chapter,
            clean_text,
        })
    }

    pub fn extract_metadata(&self, pdf_path: &Path) -> Result<BookMetadata> {
        let document = Document::load(pdf_path).context("Failed to load PDF document")?;

        let mut metadata = BookMetadata {
            title: pdf_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            author: "Unknown Author".to_string(),
            subject: None,
            creator: None,
            creation_date: Some(Utc::now()),
        };

        // Extract metadata from PDF
        if let Ok(info_dict) = document.trailer.get(b"Info")
            && let Ok(info) = info_dict.as_dict()
        {
            if let Ok(title) = info.get(b"Title")
                && let Ok(title_str) = title.as_str()
            {
                metadata.title = String::from_utf8_lossy(title_str).into();
            }

            if let Ok(author) = info.get(b"Author")
                && let Ok(author_str) = author.as_str()
            {
                metadata.author = String::from_utf8_lossy(author_str).into();
            }

            if let Ok(subject) = info.get(b"Subject")
                && let Ok(subject_str) = subject.as_str()
            {
                metadata.subject = Some(String::from_utf8_lossy(subject_str).into());
            }

            if let Ok(creator) = info.get(b"Creator")
                && let Ok(creator_str) = creator.as_str()
            {
                metadata.creator = Some(String::from_utf8_lossy(creator_str).into());
            }
        }

        Ok(metadata)
    }

    pub fn extract_text_by_pages(&self, pdf_path: &Path) -> Result<Vec<String>> {
        println!("Extracting text from PDF...");

        // Method 1: Try page-by-page extraction using lopdf
        match self.extract_pages_with_lopdf(pdf_path) {
            Ok(pages) => {
                println!(
                    "✅ Successfully extracted {} pages using page-level extraction",
                    pages.len()
                );
                return Ok(pages);
            }
            Err(e) => {
                println!("⚠️ Page-level extraction failed: {}", e);
                println!("🔄 Falling back to full-text extraction...");
            }
        }

        // Method 2: Fallback to full text extraction with intelligent splitting
        match self.extract_with_fallback_methods(pdf_path) {
            Ok(pages) => {
                println!(
                    "✅ Successfully extracted {} pages using fallback method",
                    pages.len()
                );
                Ok(pages)
            }
            Err(e) => {
                anyhow::bail!("All text extraction methods failed. Error: {}", e);
            }
        }
    }

    pub fn extract_pages_with_lopdf(&self, pdf_path: &Path) -> Result<Vec<String>> {
        let document = Document::load(pdf_path).context("Failed to load PDF with lopdf")?;

        let mut pages = Vec::new();
        let page_count = document.get_pages().len();

        if page_count == 0 {
            anyhow::bail!("PDF contains no pages");
        }

        println!("📄 Found {} pages in PDF", page_count);
        let pb = ProgressBar::new(page_count as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} pages extracted",
            )
            .unwrap(),
        );

        // Get page references
        let object_ids: Vec<_> = document.get_pages().values().cloned().collect();

        for (idx, &object_id) in object_ids.iter().enumerate() {
            pb.set_position(idx as u64);

            match self.extract_page_text(&document, object_id) {
                Ok(page_text) => {
                    let cleaned_text = if page_text.trim().is_empty() {
                        format!("[Page {} - No extractable text]", idx + 1)
                    } else {
                        page_text
                    };
                    pages.push(cleaned_text);
                }
                Err(e) => {
                    println!("⚠️ Failed to extract text from page {}: {}", idx + 1, e);
                    pages.push(format!(
                        "[Page {} - Text extraction failed: {}]",
                        idx + 1,
                        e
                    ));
                }
            }
        }

        pb.finish_with_message("Page extraction complete");

        if pages.is_empty() {
            anyhow::bail!("No text could be extracted from any page");
        }

        Ok(pages)
    }

    pub fn extract_page_text(&self, document: &Document, object_id: ObjectId) -> Result<String> {
        // Get the page object
        let page_obj = document
            .get_object(object_id)
            .context("Failed to get page object")?;

        let page_dict = page_obj
            .as_dict()
            .context("Page object is not a dictionary")?;

        // Try to extract text from the page
        let mut page_text = String::new();

        // Look for content streams
        if let Ok(contents) = page_dict.get(b"Contents") {
            match contents {
                lopdf::Object::Reference(content_id) => {
                    if let Ok(content_obj) = document.get_object(*content_id)
                        && let Ok(content_text) = self.extract_text_from_content_object(content_obj)
                    {
                        page_text.push_str(&content_text);
                    }
                }
                lopdf::Object::Array(content_array) => {
                    for content_ref in content_array {
                        if let lopdf::Object::Reference(content_id) = content_ref
                            && let Ok(content_obj) = document.get_object(*content_id)
                            && let Ok(content_text) =
                                self.extract_text_from_content_object(content_obj)
                        {
                            page_text.push_str(&content_text);
                        }
                    }
                }
                _ => {}
            }
        }

        // If no text was extracted, try alternative methods
        if page_text.trim().is_empty() {
            // Try to find text in annotations or other objects
            page_text = self
                .extract_text_from_annotations(document, page_dict)
                .unwrap_or_else(|_| String::new());
        }

        Ok(page_text)
    }

    pub fn extract_text_from_content_object(&self, content_obj: &lopdf::Object) -> Result<String> {
        match content_obj {
            lopdf::Object::Stream(stream) => {
                let content = stream
                    .decode_content()
                    .context("Failed to decode content stream")?;
                let bytes = content.encode().unwrap();

                // Parse PDF content stream for text
                self.parse_pdf_content_stream(&bytes)
            }
            _ => Ok(String::new()),
        }
    }

    pub fn parse_pdf_content_stream(&self, content: &[u8]) -> Result<String> {
        let content_str = String::from_utf8_lossy(content);
        let mut text = String::new();

        // Simple PDF content stream parser
        // Look for text objects between BT and ET operators
        let bt_et_regex = Regex::new(r"(?s)BT\s+(.*?)\s+ET").unwrap();
        let tj_regex = Regex::new(r"\[(.*?)\]\s*TJ").unwrap();
        let tj_simple_regex = Regex::new(r"\((.*?)\)\s*Tj").unwrap();

        for bt_match in bt_et_regex.find_iter(&content_str) {
            let text_block = bt_match.as_str();

            // Extract text from TJ operators (array form)
            for tj_match in tj_regex.find_iter(text_block) {
                if let Some(captures) = tj_regex.captures(tj_match.as_str())
                    && let Some(text_content) = captures.get(1)
                {
                    let cleaned = self.clean_pdf_text_content(text_content.as_str());
                    if !cleaned.is_empty() {
                        text.push_str(&cleaned);
                        text.push(' ');
                    }
                }
            }

            // Extract text from Tj operators (simple form)
            for tj_match in tj_simple_regex.find_iter(text_block) {
                if let Some(captures) = tj_simple_regex.captures(tj_match.as_str())
                    && let Some(text_content) = captures.get(1)
                {
                    let cleaned = self.clean_pdf_text_content(text_content.as_str());
                    if !cleaned.is_empty() {
                        text.push_str(&cleaned);
                        text.push(' ');
                    }
                }
            }
        }

        Ok(text)
    }

    pub fn clean_pdf_text_content(&self, text: &str) -> String {
        // Remove PDF escape sequences and clean up text
        let text = text.replace("\\(", "(");
        let text = text.replace("\\)", ")");
        let text = text.replace("\\\\", "\\");
        let text = text.replace("\\n", "\n");
        let text = text.replace("\\r", "\r");
        let text = text.replace("\\t", "\t");

        // Remove other PDF-specific artifacts
        let text = Regex::new(r"\\[0-9]{3}").unwrap().replace_all(&text, "");

        text.trim().to_string()
    }

    pub fn extract_text_from_annotations(
        &self,
        document: &Document,
        page_dict: &lopdf::Dictionary,
    ) -> Result<String> {
        let mut text = String::new();

        // Look for annotations that might contain text
        if let Ok(annots) = page_dict.get(b"Annots")
            && let lopdf::Object::Array(annot_array) = annots
        {
            for annot_ref in annot_array {
                if let lopdf::Object::Reference(annot_id) = annot_ref
                    && let Ok(annot_obj) = document.get_object(*annot_id)
                    && let Ok(annot_dict) = annot_obj.as_dict()
                {
                    // Try to extract text from Contents field
                    if let Ok(contents) = annot_dict.get(b"Contents")
                        && let Ok(content_str) = contents.as_str()
                    {
                        let content = String::from_utf8_lossy(content_str);
                        text.push_str(&content);
                        text.push('\n');
                    }
                }
            }
        }

        Ok(text)
    }

    pub fn extract_with_fallback_methods(&self, pdf_path: &Path) -> Result<Vec<String>> {
        println!("🔄 Attempting fallback text extraction methods...");

        // Method 1: pdf-extract crate (simple but reliable)
        match extract_text(pdf_path) {
            Ok(full_text) => {
                if !full_text.trim().is_empty() {
                    println!("✅ Extracted text using pdf-extract crate");
                    return Ok(self.intelligent_page_splitting(&full_text));
                }
            }
            Err(e) => {
                println!("⚠️ pdf-extract failed: {}", e);
            }
        }

        // Method 2: Try to read raw PDF content and extract text
        match self.extract_raw_text_content(pdf_path) {
            Ok(text) => {
                if !text.trim().is_empty() {
                    println!("✅ Extracted text using raw content parsing");
                    return Ok(self.intelligent_page_splitting(&text));
                }
            }
            Err(e) => {
                println!("⚠️ Raw content extraction failed: {}", e);
            }
        }

        anyhow::bail!("All fallback extraction methods failed")
    }

    pub fn extract_raw_text_content(&self, pdf_path: &Path) -> Result<String> {
        // Read the PDF file as bytes and try to extract readable text
        let pdf_bytes = fs::read(pdf_path).context("Failed to read PDF file")?;

        let pdf_string = String::from_utf8_lossy(&pdf_bytes);
        let mut extracted_text = String::new();

        // Look for text between parentheses (common in PDF text objects)
        let text_regex = Regex::new(r"\(([^)]*)\)").unwrap();
        for cap in text_regex.captures_iter(&pdf_string) {
            if let Some(text_match) = cap.get(1) {
                let text_content = text_match.as_str();
                // Filter out non-printable characters and PDF commands
                if self.is_likely_readable_text(text_content) {
                    extracted_text.push_str(text_content);
                    extracted_text.push(' ');
                }
            }
        }

        // Also look for text in literal strings
        let literal_regex = Regex::new(r"<([0-9A-Fa-f\s]+)>").unwrap();
        for cap in literal_regex.captures_iter(&pdf_string) {
            if let Some(hex_match) = cap.get(1)
                && let Ok(decoded) = self.decode_hex_string(hex_match.as_str())
                && self.is_likely_readable_text(&decoded)
            {
                extracted_text.push_str(&decoded);
                extracted_text.push(' ');
            }
        }

        Ok(extracted_text)
    }

    pub fn is_likely_readable_text(&self, text: &str) -> bool {
        if text.len() < 2 {
            return false;
        }

        let printable_chars = text
            .chars()
            .filter(|c| {
                c.is_ascii_alphanumeric() || c.is_whitespace() || ".,!?;:\"'()-".contains(*c)
            })
            .count();

        let total_chars = text.chars().count();

        // Consider text readable if more than 70% of characters are printable
        printable_chars as f64 / total_chars as f64 > 0.7
    }

    pub fn decode_hex_string(&self, hex_str: &str) -> Result<String> {
        let hex_clean = hex_str.replace(' ', "");
        let mut result = String::new();

        for chunk in hex_clean.as_bytes().chunks(2) {
            if chunk.len() == 2 {
                let hex_byte = std::str::from_utf8(chunk).context("Invalid UTF-8 in hex string")?;
                if let Ok(byte_val) = u8::from_str_radix(hex_byte, 16)
                    && (byte_val.is_ascii_graphic() || byte_val.is_ascii_whitespace())
                {
                    result.push(byte_val as char);
                }
            }
        }

        Ok(result)
    }

    pub fn intelligent_page_splitting(&self, full_text: &str) -> Vec<String> {
        println!("📊 Applying intelligent page splitting...");

        // Method 1: Split by form feed characters (most reliable)
        if full_text.contains('\x0C') {
            let pages: Vec<String> = full_text
                .split('\x0C')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if !pages.is_empty() {
                println!(
                    "✅ Split into {} pages using form feed characters",
                    pages.len()
                );
                return pages;
            }
        }

        // Method 2: Split by page indicators
        let page_indicators = [
            r"(?i)page\s+\d+",
            r"(?i)- \d+ -",
            r"(?i)^\d+\s*$",
            r"(?i)page\s+\d+\s+of\s+\d+",
        ];

        for pattern in &page_indicators {
            if let Ok(regex) = Regex::new(pattern)
                && regex.is_match(full_text)
            {
                let pages = self.split_by_pattern(full_text, pattern);
                if pages.len() > 1 {
                    println!(
                        "✅ Split into {} pages using pattern: {}",
                        pages.len(),
                        pattern
                    );
                    return pages;
                }
            }
        }

        // Method 3: Split by chapter or section markers
        let structure_patterns = [
            r"(?i)^(chapter|ch\.?)\s+\d+",
            r"(?i)^(section|sec\.?)\s+\d+",
            r"(?i)^(part|pt\.?)\s+\d+",
            r"(?i)^\d+\.\s+[A-Z]",
        ];

        for pattern in &structure_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                let matches: Vec<_> = regex.find_iter(full_text).collect();
                if matches.len() > 2 {
                    let pages = self.split_by_structural_markers(full_text, &matches);
                    if pages.len() > 1 {
                        println!(
                            "✅ Split into {} pages using structural markers",
                            pages.len()
                        );
                        return pages;
                    }
                }
            }
        }

        // Method 4: Intelligent content-based splitting
        let pages = self.content_based_page_splitting(full_text);
        if pages.len() > 1 {
            println!(
                "✅ Split into {} pages using content-based analysis",
                pages.len()
            );
            return pages;
        }

        // Method 5: Fallback to character count estimation
        println!("⚠️ Using fallback character-based splitting");
        self.character_based_splitting(full_text)
    }

    pub fn split_by_pattern(&self, text: &str, pattern: &str) -> Vec<String> {
        if let Ok(regex) = Regex::new(pattern) {
            let parts: Vec<&str> = regex.split(text).collect();
            return parts
                .into_iter()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
        vec![text.to_string()]
    }

    pub fn split_by_structural_markers(&self, text: &str, matches: &[regex::Match]) -> Vec<String> {
        let mut pages = Vec::new();
        let mut last_end = 0;

        for (i, match_obj) in matches.iter().enumerate() {
            if i > 0 {
                let page_content = &text[last_end..match_obj.start()];
                if !page_content.trim().is_empty() {
                    pages.push(page_content.trim().to_string());
                }
            }
            last_end = match_obj.start();
        }

        // Add the last page
        if last_end < text.len() {
            let last_content = &text[last_end..];
            if !last_content.trim().is_empty() {
                pages.push(last_content.trim().to_string());
            }
        }

        pages
    }

    pub fn content_based_page_splitting(&self, text: &str) -> Vec<String> {
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() < 10 {
            return vec![text.to_string()];
        }

        let mut pages = Vec::new();
        let mut current_page = Vec::new();
        let mut line_count = 0;
        let estimated_lines_per_page = 40; // Reasonable estimate

        for line in lines {
            current_page.push(line);
            line_count += 1;

            // Check for natural break points
            if line_count >= estimated_lines_per_page {
                // Look for a good break point (empty line, short line, etc.)
                if (line.trim().is_empty() || line.trim().len() < 20) && !current_page.is_empty() {
                    pages.push(current_page.join("\n"));
                    current_page.clear();
                    line_count = 0;
                }
            }

            // Force break if page gets too long
            if line_count > estimated_lines_per_page * 2 && !current_page.is_empty() {
                pages.push(current_page.join("\n"));
                current_page.clear();
                line_count = 0;
            }
        }

        // Add remaining content
        if !current_page.is_empty() {
            pages.push(current_page.join("\n"));
        }

        pages
    }

    pub fn character_based_splitting(&self, text: &str) -> Vec<String> {
        // Improved character-based splitting with smarter break points
        let target_chars_per_page = 2500; // Slightly larger than original
        let max_chars_per_page = 4000; // Hard limit

        let mut pages = Vec::new();
        let mut current_pos = 0;

        while current_pos < text.len() {
            let mut end_pos = std::cmp::min(current_pos + target_chars_per_page, text.len());

            // Try to find a good break point
            if end_pos < text.len() {
                // Look for paragraph breaks within reasonable distance
                for offset in 0..=500 {
                    if end_pos + offset < text.len() {
                        let check_pos = end_pos + offset;
                        if text.chars().nth(check_pos) == Some('\n')
                            && text.chars().nth(check_pos + 1) == Some('\n')
                        {
                            end_pos = check_pos;
                            break;
                        }
                    }

                    if end_pos > offset {
                        let check_pos = end_pos - offset;
                        if text.chars().nth(check_pos) == Some('\n')
                            && text.chars().nth(check_pos + 1) == Some('\n')
                        {
                            end_pos = check_pos;
                            break;
                        }
                    }
                }

                // If no paragraph break found, look for sentence endings
                if end_pos == current_pos + target_chars_per_page {
                    for offset in 0..=200 {
                        if end_pos + offset < text.len() {
                            let check_pos = end_pos + offset;
                            let char_at_pos = text.chars().nth(check_pos);
                            if matches!(char_at_pos, Some('.') | Some('!') | Some('?'))
                                && let Some(next_char) = text.chars().nth(check_pos + 1)
                                && next_char.is_whitespace()
                            {
                                end_pos = check_pos + 1;
                                break;
                            }
                        }
                    }
                }
            }

            // Ensure we don't exceed maximum page size
            end_pos = std::cmp::min(end_pos, current_pos + max_chars_per_page);

            let page_text = &text[current_pos..end_pos];
            if !page_text.trim().is_empty() {
                pages.push(page_text.trim().to_string());
            }

            current_pos = end_pos;
        }

        pages
    }

    pub fn clean_text_content(&self, text: &str) -> String {
        if !self.clean_text {
            return text.to_string();
        }

        let text = text.replace('\r', "");
        let text = Regex::new(r"\n\s*\n\s*\n+")
            .unwrap()
            .replace_all(&text, "\n\n");
        let text = Regex::new(r"[ \t]+").unwrap().replace_all(&text, " ");
        let text = Regex::new(r"\n ").unwrap().replace_all(&text, "\n");

        text.trim().to_string()
    }

    pub fn detect_chapters(&self, pages: &[String]) -> Vec<Chapter> {
        let mut chapters = Vec::new();
        let mut current_chapter: Option<(String, String, usize)> = None;

        let pb = ProgressBar::new(pages.len() as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} pages",
            )
            .unwrap(),
        );

        for (page_idx, page_text) in pages.iter().enumerate() {
            pb.set_position(page_idx as u64);

            let cleaned_text = self.clean_text_content(page_text);
            let lines: Vec<&str> = cleaned_text.lines().collect();

            // Look for chapter headers in the first few lines of each page
            let mut found_chapter = false;
            for (line_idx, line) in lines.iter().take(10).enumerate() {
                if self.chapter_regex.is_match(line.trim()) {
                    // Finalize previous chapter
                    if let Some((title, content, start_page)) = current_chapter.take()
                        && content.len() >= self.min_chapter_length
                    {
                        chapters.push(Chapter {
                            title,
                            content: self.clean_text_content(&content),
                            page_start: start_page,
                            page_end: page_idx.saturating_sub(1),
                        });
                    }

                    // Start new chapter
                    let chapter_title = line.trim().to_string();
                    let chapter_content = if line_idx + 1 < lines.len() {
                        lines[line_idx + 1..].join("\n")
                    } else {
                        String::new()
                    };

                    current_chapter = Some((chapter_title, chapter_content, page_idx));
                    found_chapter = true;
                    break;
                }
            }

            // If no new chapter found, add content to current chapter
            if !found_chapter && let Some((_, ref mut content, _)) = current_chapter {
                if !content.is_empty() {
                    content.push('\n');
                }
                content.push_str(&cleaned_text);
            }

            // Check if we should split chapter due to page limit
            if self.max_pages_per_chapter > 0
                && let Some((title, content, start_page)) = &current_chapter
                && page_idx - start_page >= self.max_pages_per_chapter
            {
                chapters.push(Chapter {
                    title: format!("{} (Part {})", title, chapters.len() + 1),
                    content: self.clean_text_content(content),
                    page_start: *start_page,
                    page_end: page_idx,
                });

                current_chapter = Some((
                    format!("{} (Part {})", title, chapters.len() + 1),
                    String::new(),
                    page_idx + 1,
                ));
            }
        }

        // Finalize last chapter
        if let Some((title, content, start_page)) = current_chapter
            && content.len() >= self.min_chapter_length
        {
            chapters.push(Chapter {
                title,
                content: self.clean_text_content(&content),
                page_start: start_page,
                page_end: pages.len().saturating_sub(1),
            });
        }

        pb.finish_with_message("Chapter detection complete");

        // If no chapters detected, create one chapter with all content
        if chapters.is_empty() {
            let full_content = pages.join("\n\n");
            if full_content.len() >= self.min_chapter_length {
                chapters.push(Chapter {
                    title: "Full Document".to_string(),
                    content: self.clean_text_content(&full_content),
                    page_start: 0,
                    page_end: pages.len().saturating_sub(1),
                });
            }
        }

        chapters
    }
}
