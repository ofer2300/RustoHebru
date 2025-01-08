use crate::language_detection::Language;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationRequest {
    pub text: String,
    pub source_language: Language,
    pub target_language: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    pub original_text: String,
    pub translated_text: String,
    pub source_language: Language,
    pub target_language: Language,
    pub segments: Vec<TranslationSegment>,
    pub manual_edits: HashMap<String, String>,
    pub status: TranslationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationSegment {
    pub original: String,
    pub translated: String,
    pub confidence: f64,
    pub alternatives: Vec<String>,
    pub has_manual_edit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TranslationStatus {
    Automatic,
    ManuallyEdited,
    InReview,
}

pub struct TranslationEngine {
    translation_memory: HashMap<String, String>,
    custom_dictionary: HashMap<String, String>,
    manual_edits: HashMap<String, String>,
}

impl TranslationEngine {
    pub fn new() -> Self {
        Self {
            translation_memory: HashMap::new(),
            custom_dictionary: Self::load_custom_dictionary(),
            manual_edits: HashMap::new(),
        }
    }

    pub async fn translate(&self, request: TranslationRequest) -> Result<TranslationResult> {
        let segments = self.split_into_segments(&request.text);
        let mut translated_segments = Vec::new();
        
        for segment in segments {
            let mut translated = self.translate_segment(&segment, &request)?;
            
            // בדיקה אם קיים תרגום ידני
            if let Some(manual_edit) = self.manual_edits.get(&segment) {
                translated.translated = manual_edit.clone();
                translated.has_manual_edit = true;
            }
            
            translated_segments.push(translated);
        }
        
        let translated_text = translated_segments.iter()
            .map(|s| s.translated.clone())
            .collect::<Vec<_>>()
            .join(" ");
            
        Ok(TranslationResult {
            original_text: request.text,
            translated_text,
            source_language: request.source_language,
            target_language: request.target_language,
            segments: translated_segments,
            manual_edits: self.manual_edits.clone(),
            status: if translated_segments.iter().any(|s| s.has_manual_edit) {
                TranslationStatus::ManuallyEdited
            } else {
                TranslationStatus::Automatic
            },
        })
    }
    
    pub fn apply_manual_edit(&mut self, original: String, edited: String) -> Result<()> {
        if original.trim().is_empty() {
            return Err(anyhow!("Original text cannot be empty"));
        }
        self.manual_edits.insert(original, edited);
        Ok(())
    }
    
    pub fn get_translation_alternatives(&self, text: &str, count: usize) -> Vec<String> {
        let mut alternatives = Vec::new();
        
        // חיפוש בזיכרון התרגום
        if let Some(tm_translation) = self.translation_memory.get(text) {
            alternatives.push(tm_translation.clone());
        }
        
        // חיפוש במילון המותאם אישית
        if let Some(dict_translation) = self.custom_dictionary.get(text) {
            alternatives.push(dict_translation.clone());
        }
        
        // הוספת וריאציות נוספות
        let variations = self.generate_variations(text);
        alternatives.extend(variations);
        
        alternatives.truncate(count);
        alternatives
    }
    
    fn split_into_segments(&self, text: &str) -> Vec<String> {
        text.split(|c| c == '.' || c == '!' || c == '?' || c == ';' || c == '\n')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect()
    }
    
    fn translate_segment(&self, text: &str, request: &TranslationRequest) -> Result<TranslationSegment> {
        // ניסיון למצוא תרגום בזיכרון התרגום
        if let Some(cached) = self.translation_memory.get(text) {
            return Ok(TranslationSegment {
                original: text.to_string(),
                translated: cached.clone(),
                confidence: 1.0,
                alternatives: self.get_translation_alternatives(text, 3),
                has_manual_edit: false,
            });
        }
        
        // ניסיון למצוא תרגום במילון המותאם אישית
        if let Some(custom) = self.custom_dictionary.get(text) {
            return Ok(TranslationSegment {
                original: text.to_string(),
                translated: custom.clone(),
                confidence: 0.9,
                alternatives: self.get_translation_alternatives(text, 3),
                has_manual_edit: false,
            });
        }
        
        // תרגום אוטומטי
        let translated = match (request.source_language, request.target_language) {
            (Language::Hebrew, Language::Russian) => self.translate_hebrew_to_russian(text)?,
            (Language::Russian, Language::Hebrew) => self.translate_russian_to_hebrew(text)?,
            _ => return Err(anyhow!("Unsupported language pair")),
        };
        
        Ok(TranslationSegment {
            original: text.to_string(),
            translated,
            confidence: 0.7,
            alternatives: self.get_translation_alternatives(text, 3),
            has_manual_edit: false,
        })
    }
    
    fn translate_hebrew_to_russian(&self, text: &str) -> Result<String> {
        // כאן יש להוסיף את הלוגיקה של התרגום מעברית לרוסית
        Ok(text.to_string()) // זמני
    }
    
    fn translate_russian_to_hebrew(&self, text: &str) -> Result<String> {
        // כאן יש להוסיף את הלוגיקה של התרגום מרוסית לעברית
        Ok(text.to_string()) // זמני
    }
    
    fn generate_variations(&self, text: &str) -> Vec<String> {
        let mut variations = Vec::new();
        
        // הוספת וריאציות על בסיס מילון נרדפות
        if let Some(synonyms) = self.get_synonyms(text) {
            variations.extend(synonyms);
        }
        
        variations
    }
    
    fn get_synonyms(&self, text: &str) -> Option<Vec<String>> {
        // כאן יש להוסיף את הלוגיקה של מציאת מילים נרדפות
        None // זמני
    }
    
    fn load_custom_dictionary() -> HashMap<String, String> {
        // כאן יש להוסיף את הלוגיקה של טעינת המילון המותאם אישית
        HashMap::new() // זמני
    }
} 