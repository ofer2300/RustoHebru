use std::path::Path;
use anyhow::{Result, anyhow};
use office::{Excel, PowerPoint, Word};
use html2text::from_read;
use pdf_extract::extract_text;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::Read;
use encoding_rs::WINDOWS_1255;
use encoding_rs_io::DecodeReaderBytesBuilder;

#[derive(Debug, Clone)]
pub enum FileType {
    PDF,
    Word,
    Excel,
    PowerPoint,
    HTML,
    CSV,
    Text,
}

#[derive(Debug, Clone)]
pub struct ProcessedFile {
    pub content: String,
    pub file_type: FileType,
    pub metadata: FileMetadata,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub file_name: String,
    pub file_size: u64,
    pub creation_date: String,
    pub last_modified: String,
    pub author: Option<String>,
    pub title: Option<String>,
}

pub struct FileProcessor {
    max_file_size: u64,
    supported_extensions: Vec<String>,
}

impl FileProcessor {
    pub fn new() -> Self {
        let mut supported_extensions = Vec::new();
        supported_extensions.extend(vec![
            "pdf".to_string(),
            "doc".to_string(), "docx".to_string(),
            "xls".to_string(), "xlsx".to_string(),
            "ppt".to_string(), "pptx".to_string(),
            "html".to_string(), "htm".to_string(),
            "csv".to_string(),
            "txt".to_string(),
        ]);
        
        Self {
            max_file_size: 10 * 1024 * 1024, // 10MB
            supported_extensions,
        }
    }

    pub fn process_file<P: AsRef<Path>>(&self, path: P) -> Result<ProcessedFile> {
        let path = path.as_ref();
        
        // בדיקת סיומת הקובץ
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow!("Invalid file extension"))?
            .to_lowercase();
            
        if !self.supported_extensions.contains(&extension) {
            return Err(anyhow!("Unsupported file type: {}", extension));
        }
        
        // בדיקת גודל הקובץ
        let metadata = std::fs::metadata(path)?;
        if metadata.len() > self.max_file_size {
            return Err(anyhow!("File too large"));
        }
        
        // עיבוד הקובץ בהתאם לסוג
        let (content, file_type) = match extension.as_str() {
            "pdf" => (self.process_pdf(path)?, FileType::PDF),
            "doc" | "docx" => (self.process_word(path)?, FileType::Word),
            "xls" | "xlsx" => (self.process_excel(path)?, FileType::Excel),
            "ppt" | "pptx" => (self.process_powerpoint(path)?, FileType::PowerPoint),
            "html" | "htm" => (self.process_html(path)?, FileType::HTML),
            "csv" => (self.process_csv(path)?, FileType::CSV),
            "txt" => (self.process_text(path)?, FileType::Text),
            _ => return Err(anyhow!("Unsupported file type")),
        };
        
        Ok(ProcessedFile {
            content,
            file_type,
            metadata: self.extract_metadata(path)?,
        })
    }
    
    fn process_pdf<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        extract_text(path.as_ref())
            .map_err(|e| anyhow!("Failed to process PDF: {}", e))
    }
    
    fn process_word<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let doc = Word::open(path)?;
        Ok(doc.get_text()?)
    }
    
    fn process_excel<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let workbook = Excel::open(path)?;
        let mut content = String::new();
        
        for sheet in workbook.worksheets() {
            content.push_str(&format!("Sheet: {}\n", sheet.name()));
            for row in sheet.rows() {
                for cell in row {
                    content.push_str(&format!("{}\t", cell.value()));
                }
                content.push('\n');
            }
            content.push('\n');
        }
        
        Ok(content)
    }
    
    fn process_powerpoint<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let presentation = PowerPoint::open(path)?;
        let mut content = String::new();
        
        for slide in presentation.slides() {
            content.push_str(&format!("Slide {}\n", slide.number()));
            
            // חילוץ טקסט מצורות
            for shape in slide.shapes() {
                if let Some(text) = shape.text() {
                    content.push_str(&text);
                    content.push('\n');
                }
            }
            
            // חילוץ טקסט מתיבות טקסט
            for textbox in slide.textboxes() {
                content.push_str(&textbox.text());
                content.push('\n');
            }
            
            // חילוץ טקסט מהערות
            if let Some(notes) = slide.notes() {
                content.push_str("Notes:\n");
                content.push_str(&notes);
                content.push('\n');
            }
            
            content.push('\n');
        }
        
        Ok(content)
    }
    
    fn process_html<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        // המרת HTML לטקסט נקי
        let text = from_read(content.as_bytes(), 80);
        
        // טיפול בקידוד עברית
        let (cow, _encoding_used, had_errors) = WINDOWS_1255.decode(text.as_bytes());
        if had_errors {
            return Err(anyhow!("Error decoding Hebrew text"));
        }
        
        Ok(cow.into_owned())
    }
    
    fn process_csv<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let file = File::open(path)?;
        let decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1255))
            .build(file);
            
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(decoder);
            
        let mut content = String::new();
        
        // קריאת כותרות
        if let Some(headers) = rdr.headers()? {
            for header in headers {
                content.push_str(header);
                content.push('\t');
            }
            content.push('\n');
        }
        
        // קריאת נתונים
        for result in rdr.records() {
            let record = result?;
            for field in &record {
                content.push_str(field);
                content.push('\t');
            }
            content.push('\n');
        }
        
        Ok(content)
    }
    
    fn process_text<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let file = File::open(path)?;
        let decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1255))
            .build(file);
            
        let mut content = String::new();
        decoder.read_to_string(&mut content)?;
        
        Ok(content)
    }
    
    fn extract_metadata<P: AsRef<Path>>(&self, path: P) -> Result<FileMetadata> {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path)?;
        
        Ok(FileMetadata {
            file_name: path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| anyhow!("Invalid filename"))?
                .to_string(),
            file_size: metadata.len(),
            creation_date: metadata.created()?
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs()
                .to_string(),
            last_modified: metadata.modified()?
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs()
                .to_string(),
            author: None, // יש להוסיף חילוץ מחבר מהקובץ
            title: None, // יש להוסיף חילוץ כותרת מהקובץ
        })
    }
} 