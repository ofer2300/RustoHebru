use std::path::{Path, PathBuf};
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use std::collections::{HashMap, HashSet};
use std::fs;
use image::ImageFormat;
use walkdir::WalkDir;
use tesseract::Tesseract;
use image::io::Reader as ImageReader;
use crate::technical_dictionary::TechnicalDictionary;
use crate::technical_terms::TechnicalTerm;

#[derive(Debug)]
pub struct ProcessedImage {
    path: PathBuf,
    hash: String,
    processed_at: chrono::DateTime<chrono::Utc>,
    status: ProcessingStatus,
}

#[derive(Debug)]
pub enum ProcessingStatus {
    Success,
    Failed(String),
    Pending,
}

pub struct ImageProcessor {
    watch_path: PathBuf,
    dictionary: TechnicalDictionary,
    tesseract: Tesseract,
    processed_images: HashMap<PathBuf, ProcessedImage>,
    supported_formats: HashSet<String>,
    watcher: Option<Box<dyn Watcher>>,
    rx: Option<Receiver<DebouncedEvent>>,
}

impl ImageProcessor {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut supported_formats = HashSet::new();
        supported_formats.extend(vec!["png", "jpg", "jpeg", "tiff", "bmp", "gif"]);

        let mut tesseract = Tesseract::new(None, Some("rus+heb"))?;
        tesseract.set_variable("preserve_interword_spaces", "1")?;
        
        Ok(Self {
            watch_path: path.as_ref().to_path_buf(),
            dictionary: TechnicalDictionary::new(),
            tesseract,
            processed_images: HashMap::new(),
            supported_formats,
            watcher: None,
            rx: None,
        })
    }

    pub fn start_watching(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // יצירת התיקייה אם לא קיימת
        if !self.watch_path.exists() {
            fs::create_dir_all(&self.watch_path)?;
        }

        // סריקה ראשונית של תמונות קיימות
        self.process_existing_images()?;

        // הגדרת הצופה
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1))?;
        watcher.watch(&self.watch_path, RecursiveMode::Recursive)?;

        self.watcher = Some(Box::new(watcher));
        self.rx = Some(rx);

        println!("מתחיל לעקוב אחר תיקיית התמונות: {:?}", self.watch_path);
        self.watch_loop()?;

        Ok(())
    }

    fn process_existing_images(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("סורק תמונות קיימות...");
        
        for entry in WalkDir::new(&self.watch_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
                if let Some(ext) = entry.path().extension() {
                    if self.is_supported_format(ext) {
                        self.process_image(entry.path())?;
                    }
                }
        }
        
        println!("סיום סריקת תמונות קיימות");
        Ok(())
    }

    fn watch_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let rx = self.rx.as_ref().expect("Receiver not initialized");
        
        loop {
            match rx.recv() {
                Ok(event) => {
                    match event {
                        DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                            if let Some(ext) = path.extension() {
                                if self.is_supported_format(ext) {
                                    println!("זוהתה תמונה חדשה: {:?}", path);
                                    self.process_image(&path)?;
                                }
                            }
                        }
                        DebouncedEvent::Remove(path) => {
                            self.processed_images.remove(&path);
                            println!("תמונה הוסרה: {:?}", path);
                        }
                        DebouncedEvent::Rename(from, to) => {
                            if let Some(image) = self.processed_images.remove(&from) {
                                self.processed_images.insert(to.clone(), ProcessedImage {
                                    path: to,
                                    ..image
                                });
                            }
                        }
                        DebouncedEvent::Error(err, path) => {
                            println!("שגיאה בצפייה בקובץ {:?}: {:?}", path, err);
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    println!("שגיאה בצפייה בתיקייה: {:?}", e);
                    // ניסיון לאתחל מחדש את הצופה
                    self.restart_watcher()?;
                }
            }
        }
    }

    fn is_supported_format<P: AsRef<Path>>(&self, ext: P) -> bool {
        ext.as_ref()
            .to_str()
            .map(|s| self.supported_formats.contains(s.to_lowercase().as_str()))
            .unwrap_or(false)
    }

    fn restart_watcher(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("מאתחל מחדש את הצופה...");
        self.watcher = None;
        self.rx = None;
        
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1))?;
        watcher.watch(&self.watch_path, RecursiveMode::Recursive)?;
        
        self.watcher = Some(Box::new(watcher));
        self.rx = Some(rx);
        
        println!("הצופה אותחל מחדש בהצלחה");
        Ok(())
    }

    fn calculate_image_hash(&self, path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let mut hasher = blake3::Hasher::new();
        let contents = fs::read(path)?;
        hasher.update(&contents);
        Ok(hasher.finalize().to_string())
    }

    fn process_image(&mut self, image_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("מעבד תמונה: {:?}", image_path);

        // חישוב hash לזיהוי תמונות כפולות
        let hash = self.calculate_image_hash(image_path)?;
        
        // בדיקה אם התמונה כבר עובדה
        if self.processed_images.values().any(|img| img.hash == hash) {
            println!("תמונה זהה כבר עובדה בעבר");
            return Ok(());
        }

        // ניסיון לעבד את התמונה
        match self.process_image_internal(image_path) {
            Ok(()) => {
                self.processed_images.insert(image_path.to_path_buf(), ProcessedImage {
                    path: image_path.to_path_buf(),
                    hash,
                    processed_at: chrono::Utc::now(),
                    status: ProcessingStatus::Success,
                });
                println!("תמונה עובדה בהצלחה: {:?}", image_path);
            }
            Err(e) => {
                self.processed_images.insert(image_path.to_path_buf(), ProcessedImage {
                    path: image_path.to_path_buf(),
                    hash,
                    processed_at: chrono::Utc::now(),
                    status: ProcessingStatus::Failed(e.to_string()),
                });
                println!("שגיאה בעיבוד תמונה {:?}: {:?}", image_path, e);
            }
        }

        Ok(())
    }

    fn process_image_internal(&mut self, image_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // טעינת התמונה
        let img = ImageReader::open(image_path)?
            .with_guessed_format()?
            .decode()?;
        
        //�יפור איכות התמונה
        let enhanced = self.enhance_image(&img)?;
        
        // המרה לשחור-לבן עם אופטימיזציה
        let gray = self.optimize_for_ocr(&enhanced)?;
        
        // הגדרות OCR מותאמות אישית
        self.configure_tesseract_for_image(&gray)?;
        
        // OCR על התמונה
        self.tesseract.set_image_from_mem(&gray.to_bytes())?;
        let text = self.tesseract.get_text()?;

        // ניתוח הטקסט וחילוץ מונחים
        self.analyze_text(&text)?;

        Ok(())
    }

    fn enhance_image(&self, img: &image::DynamicImage) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
        use image::{ImageBuffer, Rgb};
        
        // התאמת ניגודיות
        let contrast_enhanced = imageproc::contrast::stretch_contrast(img.to_rgb8(), 10);
        
        // הסרת רעש
        let denoised = imageproc::filter::median_filter(&contrast_enhanced, 3, 3);
        
        // חידוד התמונה
        let sharpened = imageproc::filter::sharpen(&denoised);
        
        // תיקון הטיה
        let skew_angle = self.detect_skew(&sharpened)?;
        let rotated = if skew_angle.abs() > 0.5 {
            imageproc::geometric_transformations::rotate(
                &sharpened,
                -skew_angle,
                imageproc::geometric_transformations::Interpolation::Bicubic,
                Rgb([255, 255, 255])
            )
        } else {
            sharpened
        };

        Ok(image::DynamicImage::ImageRgb8(rotated))
    }

    fn detect_skew(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<f32, Box<dyn std::error::Error>> {
        // זיהוי זווית ההטיה באמצעות התמרת הוף
        let edges = imageproc::edges::canny(img, 50.0, 150.0);
        let lines = imageproc::hough::detect_lines(&edges, 1.0, 0.01);
        
        if lines.is_empty() {
            return Ok(0.0);
        }

        // חישוב הזווית הממוצעת
        let avg_angle = lines.iter()
            .map(|line| line.angle())
            .sum::<f32>() / lines.len() as f32;

        Ok(avg_angle)
    }

    fn optimize_for_ocr(&self, img: &image::DynamicImage) -> Result<image::GrayImage, Box<dyn std::error::Error>> {
        use image::GrayImage;
        
        // המרה לשחור-לבן עם סף דינמי
        let gray = img.to_luma8();
        let threshold = imageproc::contrast::otsu_level(&gray);
        let binary = imageproc::contrast::threshold(&gray, threshold);
        
        // הסרת רעש נקודתי
        let cleaned = imageproc::morphology::close(&binary, 
            imageproc::morphology::Norm::LInf, 2);
        
        // מילוי חורים קטנים
        let filled = imageproc::morphology::fill_holes(&cleaned);
        
        Ok(filled)
    }

    fn configure_tesseract_for_image(&mut self, img: &image::GrayImage) -> Result<(), Box<dyn std::error::Error>> {
        // הגדרות בסיסיות
        self.tesseract.set_variable("preserve_interword_spaces", "1")?;
        
        // אופטימיזציה לפי גודל הטקסט
        let (width, height) = img.dimensions();
        let dpi = self.estimate_dpi(width, height);
        self.tesseract.set_variable("user_defined_dpi", &dpi.to_string())?;
        
        // הגדרות מתקדמות
        self.tesseract.set_variable("tessedit_pageseg_mode", "1")?; // אוטומטי עם כיוון
        self.tesseract.set_variable("tessedit_ocr_engine_mode", "2")?; // LSTM בלבד
        self.tesseract.set_variable("tessedit_char_whitelist", 
            "אבגדהוזחטיכלמנסעפצקרשתםןץףך\
             АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ\
             абвгдеёжзийклмнопрстуфхцчшщъыьэюя")?;
        
        Ok(())
    }

    fn estimate_dpi(&self, width: u32, height: u32) -> u32 {
        // הערכת DPI לפי גודל התמונה
        let area = width * height;
        match area {
            0..=1_000_000 => 150,    // תמונות קטנות
            1_000_001..=4_000_000 => 300,  // תמונות בינוניות
            _ => 600,  // תמונות גדולות
        }
    }

    pub fn get_processing_stats(&self) -> ProcessingStats {
        let total = self.processed_images.len();
        let successful = self.processed_images.values()
            .filter(|img| matches!(img.status, ProcessingStatus::Success))
            .count();
        let failed = self.processed_images.values()
            .filter(|img| matches!(img.status, ProcessingStatus::Failed(_)))
            .count();
        
        ProcessingStats {
            total,
            successful,
            failed,
        }
    }
}

#[derive(Debug)]
pub struct ProcessingStats {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
} 