use iced::{Application, Settings};
use crate::gui::TranslatorGui;
use crate::file_processor::FileProcessor;
use crate::security::SecurityManager;
use crate::translation::TranslationEngine;
use std::sync::Arc;
use std::path::Path;
use image_processor::ImageProcessor;
use error::{Result, ErrorExt};
use log::{info, warn, error};
use std::env;

mod gui;
mod file_processor;
mod security;
mod translation;
mod language_detection;
mod fonts;
mod metadata;
mod technical_terms;
mod technical_dictionary;
mod image_processor;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    // הגדרת לוגים
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    info!("מתחיל את מערכת המעקב אחר תמונות...");
    
    let watch_path = Path::new(r"C:\Users\user\Desktop\אימון לרוסית\תפורות");
    
    if !watch_path.exists() {
        info!("התיקייה לא קיימת: {:?}", watch_path);
        info!("יוצר את התיקייה...");
        std::fs::create_dir_all(watch_path)
            .with_context(|| format!("נכשל ביצירת תיקייה {:?}", watch_path))?;
    }
    
    let mut processor = ImageProcessor::new(watch_path)
        .with_context(|| "נכשל באתחול מעבד התמונות")?;
    
    info!("מתחיל לעקוב אחר התיקייה: {:?}", watch_path);
    info!("המערכת תזהה ותעבד אוטומטית כל תמונה חדשה שתתווסף לתיקייה.");
    info!("לחץ Ctrl+C לסיום.");

    // טיפול בסיום חלק
    ctrlc::set_handler(move || {
        info!("התקבל אות סיום, מסיים...");
        std::process::exit(0);
    }).expect("שגיאה בהגדרת מטפל סיום");
    
    processor.start_watching()
        .with_context(|| "שגיאה במעקב אחר תיקייה")?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;

    #[tokio::test]
    async fn test_image_processor() -> Result<()> {
        // יצירת תיקייה זמנית לבדיקות
        let temp_dir = TempDir::new()?;
        let mut processor = ImageProcessor::new(temp_dir.path())?;

        // יצירת תמונת בדיקה
        let test_image_path = temp_dir.path().join("test.png");
        let mut test_image = File::create(&test_image_path)?;
        // TODO: ליצור תמונת בדיקה אמיתית
        write!(test_image, "test data")?;

        // בדיקת עיבוד תמונה
        processor.process_image(&test_image_path)?;

        // בדיקת סטטיסטיקה
        let stats = processor.get_processing_stats();
        assert_eq!(stats.total, 1);
        assert!(stats.successful > 0 || stats.failed > 0);

        Ok(())
    }
} 