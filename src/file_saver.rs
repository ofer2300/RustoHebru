use std::path::{Path, PathBuf};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::Write;
use crate::file_processor::{FileType, ProcessedFile};
use crate::metadata::FileMetadata;
use crate::security::SecurityManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveOptions {
    pub output_dir: Option<PathBuf>,
    pub file_name_template: Option<String>,
    pub preserve_original: bool,
    pub create_backup: bool,
    pub include_metadata: bool,
    pub compress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveResult {
    pub saved_path: PathBuf,
    pub original_file: PathBuf,
    pub backup_path: Option<PathBuf>,
    pub metadata_path: Option<PathBuf>,
    pub file_size: u64,
    pub save_date: DateTime<Utc>,
}

pub struct FileSaver {
    security_manager: SecurityManager,
    default_output_dir: PathBuf,
    backup_dir: PathBuf,
}

impl FileSaver {
    pub fn new(security_manager: SecurityManager) -> Self {
        let default_output_dir = PathBuf::from("output");
        let backup_dir = PathBuf::from("backups");
        
        // יצירת תיקיות אם לא קיימות
        fs::create_dir_all(&default_output_dir).unwrap_or_default();
        fs::create_dir_all(&backup_dir).unwrap_or_default();
        
        Self {
            security_manager,
            default_output_dir,
            backup_dir,
        }
    }
    
    pub fn save_translated_file(
        &self,
        original_file: &ProcessedFile,
        translated_text: &str,
        options: SaveOptions,
    ) -> Result<SaveResult> {
        // הכנת נתיב השמירה
        let output_dir = options.output_dir
            .unwrap_or_else(|| self.default_output_dir.clone());
            
        fs::create_dir_all(&output_dir)?;
        
        // יצירת שם קובץ
        let file_name = if let Some(template) = options.file_name_template {
            self.generate_file_name(&template, original_file)
        } else {
            self.generate_default_file_name(original_file)
        };
        
        let output_path = output_dir.join(&file_name);
        
        // יצירת גיבוי אם נדרש
        let backup_path = if options.create_backup {
            let backup_file_name = format!(
                "{}_{}",
                file_name.file_stem().unwrap().to_string_lossy(),
                Utc::now().timestamp()
            );
            let backup_path = self.backup_dir.join(backup_file_name);
            fs::copy(&output_path, &backup_path).ok();
            Some(backup_path)
        } else {
            None
        };
        
        // שמירת הקובץ המתורגם
        self.write_translated_file(&output_path, original_file, translated_text)?;
        
        // שמירת מטא-דאטה
        let metadata_path = if options.include_metadata {
            let metadata = FileMetadata {
                original_path: original_file.metadata.file_name.clone(),
                translation_date: Utc::now(),
                file_type: original_file.file_type.clone(),
                original_size: original_file.metadata.file_size,
                translated_size: fs::metadata(&output_path)?.len(),
                custom_properties: Default::default(),
            };
            
            let metadata_path = output_path.with_extension("metadata.json");
            let metadata_file = File::create(&metadata_path)?;
            serde_json::to_writer_pretty(metadata_file, &metadata)?;
            Some(metadata_path)
        } else {
            None
        };
        
        Ok(SaveResult {
            saved_path: output_path.clone(),
            original_file: PathBuf::from(&original_file.metadata.file_name),
            backup_path,
            metadata_path,
            file_size: fs::metadata(&output_path)?.len(),
            save_date: Utc::now(),
        })
    }
    
    fn write_translated_file(
        &self,
        path: &Path,
        original_file: &ProcessedFile,
        translated_text: &str,
    ) -> Result<()> {
        match original_file.file_type {
            FileType::PDF => self.write_pdf(path, translated_text)?,
            FileType::Word => self.write_word(path, translated_text)?,
            FileType::Excel => self.write_excel(path, translated_text)?,
            FileType::PowerPoint => self.write_powerpoint(path, translated_text)?,
            FileType::HTML => self.write_html(path, translated_text)?,
            FileType::CSV => self.write_csv(path, translated_text)?,
            FileType::Text => self.write_text(path, translated_text)?,
        }
        
        Ok(())
    }
    
    fn write_pdf(&self, path: &Path, content: &str) -> Result<()> {
        use printpdf::{PdfDocument, PdfDocumentReference, Mm, Point};
        
        let (doc, page1, layer1) = PdfDocument::new(
            "Translated Document",
            Mm(210.0),
            Mm(297.0),
            "Layer 1",
        );
        
        let current_layer = doc.get_page(page1).get_layer(layer1);
        
        // הוספת טקסט לPDF
        current_layer.begin_text_section();
        current_layer.set_font("Helvetica", 12.0);
        current_layer.set_text_cursor(Mm(10.0), Mm(287.0));
        current_layer.write_text(content, &doc);
        current_layer.end_text_section();
        
        doc.save(&mut File::create(path)?)?;
        Ok(())
    }
    
    fn write_word(&self, path: &Path, content: &str) -> Result<()> {
        use docx_rs::*;
        
        let path = path.with_extension("docx");
        let mut doc = Docx::new();
        
        // הוספת טקסט למסמך
        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text(content)));
        
        let file = File::create(path)?;
        doc.build().pack(file)?;
        
        Ok(())
    }
    
    fn write_excel(&self, path: &Path, content: &str) -> Result<()> {
        use calamine::{DataType, Xlsx, Writer, WorkbookWriter};
        
        let path = path.with_extension("xlsx");
        let mut workbook = WorkbookWriter::new();
        
        // יצירת גיליון חדש
        let sheet_name = "Translated";
        workbook.create_sheet(sheet_name);
        
        // הוספת תוכן לגיליון
        for (row_idx, line) in content.lines().enumerate() {
            workbook.write_cell((row_idx as u32, 0), DataType::String(line.to_string()))?;
        }
        
        workbook.close()?;
        
        Ok(())
    }
    
    fn write_powerpoint(&self, path: &Path, content: &str) -> Result<()> {
        use pptx::*;
        
        let path = path.with_extension("pptx");
        let mut presentation = Presentation::new();
        
        // יצירת שקופית חדשה
        let mut slide = Slide::new();
        
        // הוספת תיבת טקסט
        let textbox = TextBox::new()
            .set_text(content)
            .set_position(Point::new(50.0, 50.0))
            .set_size(Size::new(600.0, 400.0));
            
        slide.add_shape(textbox);
        presentation.add_slide(slide);
        
        presentation.save(path)?;
        
        Ok(())
    }
    
    fn write_html(&self, path: &Path, content: &str) -> Result<()> {
        let path = path.with_extension("html");
        let html = format!(
            r#"<!DOCTYPE html>
<html dir="rtl">
<head>
    <meta charset="UTF-8">
    <title>Translated Document</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            margin: 2em;
            line-height: 1.6;
        }}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
            content.replace('\n', "<br>")
        );
        
        fs::write(path, html)?;
        Ok(())
    }
    
    fn write_csv(&self, path: &Path, content: &str) -> Result<()> {
        let path = path.with_extension("csv");
        let mut wtr = csv::Writer::from_path(path)?;
        
        for line in content.lines() {
            wtr.write_record(&[line])?;
        }
        
        wtr.flush()?;
        Ok(())
    }
    
    fn write_text(&self, path: &Path, content: &str) -> Result<()> {
        let path = path.with_extension("txt");
        fs::write(path, content)?;
        Ok(())
    }
    
    fn generate_file_name(&self, template: &str, file: &ProcessedFile) -> PathBuf {
        let mut file_name = template.to_string();
        
        // החלפת תבניות
        file_name = file_name
            .replace("{original}", &file.metadata.file_name)
            .replace("{date}", &Utc::now().format("%Y%m%d_%H%M%S").to_string())
            .replace("{type}", &format!("{:?}", file.file_type).to_lowercase());
            
        PathBuf::from(file_name)
    }
    
    fn generate_default_file_name(&self, file: &ProcessedFile) -> PathBuf {
        let original_name = Path::new(&file.metadata.file_name)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
            
        let extension = match file.file_type {
            FileType::PDF => "pdf",
            FileType::Word => "docx",
            FileType::Excel => "xlsx",
            FileType::PowerPoint => "pptx",
            FileType::HTML => "html",
            FileType::CSV => "csv",
            FileType::Text => "txt",
        };
        
        PathBuf::from(format!(
            "{}_translated_{}.{}",
            original_name,
            Utc::now().format("%Y%m%d_%H%M%S"),
            extension
        ))
    }
} 