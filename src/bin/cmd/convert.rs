use std::path::PathBuf;

use clap::{Args, ValueEnum};

use kepub::convert::epub::EbookGenerator;
use kepub::convert::pdf::PdfProcessor;

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Epub,
    Html,
    Txt,
}

#[derive(Args, Clone, Debug)]
pub struct ConvertCmd {
    /// Input PDF file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output file path (without extension)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output format
    #[arg(short, long, default_value = "epub")]
    format: OutputFormat,

    /// Book title (extracted from PDF metadata if not provided)
    #[arg(short, long)]
    title: Option<String>,

    /// Book author (extracted from PDF metadata if not provided)
    #[arg(short, long)]
    author: Option<String>,

    /// Chapter detection pattern (regex)
    #[arg(long, default_value = r"(?i)^(chapter|ch\.?)\s*\d+")]
    chapter_pattern: String,

    /// Minimum chapter length in characters
    #[arg(long, default_value = "500")]
    min_chapter_length: usize,

    /// Maximum pages per chapter (0 for no limit)
    #[arg(long, default_value = "0")]
    max_pages_per_chapter: usize,

    /// Include page numbers in output
    #[arg(long)]
    include_page_numbers: bool,

    /// Clean up text (remove extra whitespace, fix formatting)
    #[arg(long, default_value = "true")]
    clean_text: bool,

    /// Generate table of contents
    #[arg(long, default_value = "true")]
    generate_toc: bool,
}

impl ConvertCmd {
    pub async fn exec(&self) -> anyhow::Result<()> {
        if !self.input.exists() {
            anyhow::bail!("Input PDF file does not exist: {}", self.input.display());
        }

        let output_path = self.output.clone().unwrap_or_else(|| {
            let mut path = self.input.clone();
            path.set_extension(match self.format {
                OutputFormat::Epub => "epub",
                OutputFormat::Html => "html",
                OutputFormat::Txt => "txt",
            });
            path
        });

        let processor = PdfProcessor::new(
            &self.chapter_pattern,
            self.min_chapter_length,
            self.max_pages_per_chapter,
            self.clean_text,
        )?;

        let mut metadata = processor.extract_metadata(&self.input)?;

        if let Some(title) = self.title.clone() {
            metadata.title = title;
        }

        if let Some(author) = self.author.clone() {
            metadata.author = author;
        }

        println!("Processing: {}", self.input.display());
        println!("Title: {}", metadata.title);
        println!("Author: {}", metadata.author);

        let pages = processor.extract_text_by_pages(&self.input)?;
        println!("Extracted {} pages", pages.len());

        let chapters = processor.detect_chapters(&pages);
        println!("Detected {} chapters", chapters.len());

        if chapters.is_empty() {
            anyhow::bail!(
                "No chapters detected or content too short. Try adjusting the --min-chapter-length or --chapter-pattern parameters."
            );
        }

        let generator = EbookGenerator;
        match self.format {
            OutputFormat::Epub => {
                generator.generate_epub(
                    &metadata,
                    &chapters,
                    &output_path,
                    self.include_page_numbers,
                    self.generate_toc,
                )?;
            }
            OutputFormat::Html => {
                generator.generate_html(
                    &metadata,
                    &chapters,
                    &output_path,
                    self.include_page_numbers,
                    self.generate_toc,
                )?;
            }
            OutputFormat::Txt => {
                generator.generate_txt(
                    &metadata,
                    &chapters,
                    &output_path,
                    self.include_page_numbers,
                    self.generate_toc,
                )?;
            }
        }

        println!("✅ Successfully converted PDF to ebook:");
        println!("   Output: {}", output_path.display());
        println!("   Format: {:?}", self.format);
        println!("   Chapters: {}", chapters.len());

        Ok(())
    }
}
