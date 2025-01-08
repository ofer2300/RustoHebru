use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("שגיאה בטעינת תמונה: {0}")]
    ImageLoadError(#[from] image::ImageError),

    #[error("שגיאה בזיהוי טקסט: {0}")]
    OcrError(#[from] tesseract::TesseractError),

    #[error("שגיאה בקריאת קובץ {path}: {error}")]
    FileReadError {
        path: PathBuf,
        error: std::io::Error,
    },

    #[error("שגיאה בכתיבת קובץ {path}: {error}")]
    FileWriteError {
        path: PathBuf,
        error: std::io::Error,
    },

    #[error("פורמט קובץ לא נתמך: {0}")]
    UnsupportedFormat(String),

    #[error("שגיאה במעקב אחר תיקייה: {0}")]
    WatchError(#[from] notify::Error),

    #[error("שגיאה בחישוב hash: {0}")]
    HashError(String),

    #[error("שגיאה כללית: {0}")]
    GeneralError(String),
}

pub type Result<T> = std::result::Result<T, ProcessingError>;

impl From<std::io::Error> for ProcessingError {
    fn from(error: std::io::Error) -> Self {
        ProcessingError::GeneralError(error.to_string())
    }
}

impl From<String> for ProcessingError {
    fn from(error: String) -> Self {
        ProcessingError::GeneralError(error)
    }
}

pub trait ErrorExt<T> {
    fn with_context<F, C>(self, context: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: std::fmt::Display;
}

impl<T, E> ErrorExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_context<F, C>(self, context: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: std::fmt::Display,
    {
        self.map_err(|error| {
            ProcessingError::GeneralError(format!("{}: {}", context(), error))
        })
    }
} 