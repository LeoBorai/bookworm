use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use epub_builder::{EpubBuilder, EpubContent, ZipLibrary};
use indicatif::{ProgressBar, ProgressStyle};

use crate::convert::{BookMetadata, Chapter};

pub struct EbookGenerator;

impl EbookGenerator {
    pub fn generate_epub(
        &self,
        metadata: &BookMetadata,
        chapters: &[Chapter],
        output_path: &Path,
        include_page_numbers: bool,
        generate_toc: bool,
    ) -> Result<()> {
        println!("Generating EPUB...");

        let mut builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();

        // Set metadata
        builder.metadata("title", &metadata.title).unwrap();
        builder.metadata("author", &metadata.author).unwrap();

        if let Some(subject) = &metadata.subject {
            builder.metadata("subject", subject).unwrap();
        }

        builder.metadata("lang", "en").unwrap();
        builder
            .metadata("generator", "pdf-to-ebook Rust converter")
            .unwrap();

        // Add CSS for better formatting
        let css_content = r#"
            body {
                font-family: "Times New Roman", serif;
                line-height: 1.6;
                margin: 1em;
            }

            h1, h2, h3 {
                color: #333;
                margin-top: 1.5em;
                margin-bottom: 0.5em;
            }

            h1 {
                font-size: 1.8em;
                border-bottom: 2px solid #ccc;
                padding-bottom: 0.3em;
            }

            h2 {
                font-size: 1.4em;
            }

            p {
                margin: 0.8em 0;
                text-align: justify;
            }

            .page-number {
                font-size: 0.8em;
                color: #666;
                margin: 1em 0;
                text-align: center;
            }

            .chapter-title {
                text-align: center;
                margin: 2em 0;
                font-weight: bold;
            }
        "#;

        builder.stylesheet(css_content.as_bytes()).unwrap();

        // Add chapters
        let pb = ProgressBar::new(chapters.len() as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.green/blue} {pos}/{len} chapters",
            )
            .unwrap(),
        );

        for (idx, chapter) in chapters.iter().enumerate() {
            pb.set_position(idx as u64);

            let mut content = String::new();
            content.push_str(&format!(
                r#"<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>{}</title>
    <link rel="stylesheet" type="text/css" href="stylesheet.css"/>
</head>
<body>
    <div class="chapter-title">
        <h1>{}</h1>
    </div>
"#,
                html_escape::encode_text(&chapter.title),
                html_escape::encode_text(&chapter.title)
            ));

            if include_page_numbers {
                content.push_str(&format!(
                    r#"    <div class="page-number">Pages: {} - {}</div>"#,
                    chapter.page_start + 1,
                    chapter.page_end + 1
                ));
            }

            // Convert text to HTML paragraphs
            let paragraphs: Vec<&str> = chapter.content.split("\n\n").collect();
            for paragraph in paragraphs {
                if !paragraph.trim().is_empty() {
                    content.push_str(&format!(
                        "    <p>{}</p>\n",
                        html_escape::encode_text(paragraph.trim()).replace('\n', "<br/>")
                    ));
                }
            }

            content.push_str("</body>\n</html>");

            let filename = format!("chapter_{:03}.xhtml", idx + 1);
            builder
                .add_content(
                    EpubContent::new(&filename, content.as_bytes())
                        .title(&chapter.title)
                        .reftype(epub_builder::ReferenceType::Text),
                )
                .unwrap();
        }

        pb.finish_with_message("Chapters added to EPUB");

        // Generate the EPUB file
        let mut epub_data = Vec::new();
        builder.generate(&mut epub_data).unwrap();
        fs::write(output_path, &mut epub_data).context("Failed to write EPUB file")?;

        Ok(())
    }

    pub fn generate_html(
        &self,
        metadata: &BookMetadata,
        chapters: &[Chapter],
        output_path: &Path,
        include_page_numbers: bool,
        generate_toc: bool,
    ) -> Result<()> {
        println!("Generating HTML...");

        let mut content = String::new();

        // HTML header
        content.push_str(&format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: "Times New Roman", serif;
            line-height: 1.6;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background-color: #fff;
        }}

        h1 {{
            color: #333;
            border-bottom: 2px solid #ccc;
            padding-bottom: 10px;
            text-align: center;
        }}

        h2 {{
            color: #666;
            margin-top: 2em;
            margin-bottom: 1em;
        }}

        .book-title {{
            text-align: center;
            font-size: 2.5em;
            margin: 1em 0;
        }}

        .author {{
            text-align: center;
            font-size: 1.2em;
            color: #666;
            margin-bottom: 2em;
        }}

        .toc {{
            background-color: #f9f9f9;
            padding: 20px;
            margin: 2em 0;
            border-radius: 5px;
        }}

        .toc ul {{
            list-style-type: none;
            padding-left: 0;
        }}

        .toc li {{
            margin: 0.5em 0;
        }}

        .toc a {{
            text-decoration: none;
            color: #0066cc;
        }}

        .toc a:hover {{
            text-decoration: underline;
        }}

        .chapter {{
            margin: 3em 0;
            padding-top: 2em;
            border-top: 1px solid #eee;
        }}

        .page-info {{
            font-size: 0.9em;
            color: #999;
            margin-bottom: 1em;
        }}

        p {{
            margin: 1em 0;
            text-align: justify;
        }}
    </style>
</head>
<body>
    <div class="book-title">{}</div>
    <div class="author">by {}</div>
"#,
            html_escape::encode_text(&metadata.title),
            html_escape::encode_text(&metadata.title),
            html_escape::encode_text(&metadata.author)
        ));

        // Table of contents
        if generate_toc {
            content.push_str(
                r#"    <div class="toc">
        <h2>Table of Contents</h2>
        <ul>
"#,
            );

            for (idx, chapter) in chapters.iter().enumerate() {
                content.push_str(&format!(
                    r#"            <li><a href="\#chapter_{}">{}</a></li>"#,
                    idx + 1,
                    html_escape::encode_text(&chapter.title)
                ));
            }

            content.push_str("        </ul>\n    </div>\n");
        }

        // Add chapters
        for (idx, chapter) in chapters.iter().enumerate() {
            content.push_str(&format!(
                r#"    <div class="chapter" id="chapter_{}">
        <h2>{}</h2>
"#,
                idx + 1,
                html_escape::encode_text(&chapter.title)
            ));

            if include_page_numbers {
                content.push_str(&format!(
                    r#"        <div class="page-info">Pages: {} - {}</div>
"#,
                    chapter.page_start + 1,
                    chapter.page_end + 1
                ));
            }

            // Convert text to HTML paragraphs
            let paragraphs: Vec<&str> = chapter.content.split("\n\n").collect();
            for paragraph in paragraphs {
                if !paragraph.trim().is_empty() {
                    content.push_str(&format!(
                        "        <p>{}</p>\n",
                        html_escape::encode_text(paragraph.trim()).replace('\n', "<br/>")
                    ));
                }
            }

            content.push_str("    </div>\n");
        }

        content.push_str("</body>\n</html>");

        fs::write(output_path, content).context("Failed to write HTML file")?;

        Ok(())
    }

    pub fn generate_txt(
        &self,
        metadata: &BookMetadata,
        chapters: &[Chapter],
        output_path: &Path,
        include_page_numbers: bool,
        generate_toc: bool,
    ) -> Result<()> {
        println!("Generating TXT...");

        let mut content = String::new();

        // Title and author
        content.push_str(&format!("{}\n", metadata.title.to_uppercase()));
        content.push_str(&format!("by {}\n", metadata.author));
        content.push_str(&format!("{}\n\n", "=".repeat(60)));

        // Table of contents
        if generate_toc {
            content.push_str("TABLE OF CONTENTS\n");
            content.push_str(&format!("{}\n\n", "-".repeat(20)));

            for (idx, chapter) in chapters.iter().enumerate() {
                content.push_str(&format!("{}. {}\n", idx + 1, chapter.title));
            }
            content.push_str("\n\n");
        }

        // Add chapters
        for (idx, chapter) in chapters.iter().enumerate() {
            content.push_str(&format!(
                "\n\nCHAPTER {}: {}\n",
                idx + 1,
                chapter.title.to_uppercase()
            ));
            content.push_str(&format!("{}\n", "=".repeat(60)));

            if include_page_numbers {
                content.push_str(&format!(
                    "Pages: {} - {}\n\n",
                    chapter.page_start + 1,
                    chapter.page_end + 1
                ));
            }

            content.push_str(&chapter.content);
            content.push_str("\n\n");
        }

        fs::write(output_path, content).context("Failed to write TXT file")?;

        Ok(())
    }
}
