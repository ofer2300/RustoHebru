use whatlang::{Lang, Script, detect_lang, detect_script};
use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Hebrew,
    Russian,
    Unknown,
}

pub struct LanguageDetector {
    min_confidence: f64,
    script_weights: HashMap<Script, f64>,
}

impl LanguageDetector {
    pub fn new() -> Self {
        let mut script_weights = HashMap::new();
        script_weights.insert(Script::Hebrew, 1.2);
        script_weights.insert(Script::Cyrillic, 1.2);
        
        Self {
            min_confidence: 0.8,
            script_weights,
        }
    }

    pub fn detect_language(&self, text: &str) -> Language {
        if text.trim().is_empty() {
            return Language::Unknown;
        }

        // חלוקה לקטעים לפי סקריפט
        let mut hebrew_text = String::new();
        let mut russian_text = String::new();
        let mut current_script = None;
        let mut current_text = String::new();

        for grapheme in text.graphemes(true) {
            if let Some(script) = detect_script(grapheme) {
                match script {
                    Script::Hebrew => hebrew_text.push_str(grapheme),
                    Script::Cyrillic => russian_text.push_str(grapheme),
                    _ => continue,
                }
            }
        }

        // חישוב אחוזים
        let total_len = hebrew_text.len() + russian_text.len();
        if total_len == 0 {
            return Language::Unknown;
        }

        let hebrew_ratio = (hebrew_text.len() as f64 / total_len as f64) * 
            self.script_weights.get(&Script::Hebrew).unwrap_or(&1.0);
        let russian_ratio = (russian_text.len() as f64 / total_len as f64) * 
            self.script_weights.get(&Script::Cyrillic).unwrap_or(&1.0);

        // בדיקת מילים נפוצות
        let hebrew_keywords = ["של", "את", "על", "עם", "זה", "גם", "כל", "או", "אבל", "כי"];
        let russian_keywords = ["и", "в", "не", "на", "с", "по", "для", "от", "из", "при"];

        let mut hebrew_keyword_count = 0;
        let mut russian_keyword_count = 0;

        for word in text.split_whitespace() {
            if hebrew_keywords.contains(&word) {
                hebrew_keyword_count += 1;
            }
            if russian_keywords.contains(&word) {
                russian_keyword_count += 1;
            }
        }

        let keyword_ratio = if hebrew_keyword_count + russian_keyword_count > 0 {
            let hebrew_weight = hebrew_keyword_count as f64 * 0.2;
            let russian_weight = russian_keyword_count as f64 * 0.2;
            (hebrew_ratio + hebrew_weight, russian_ratio + russian_weight)
        } else {
            (hebrew_ratio, russian_ratio)
        };

        // קבלת החלטה סופית
        if keyword_ratio.0 > self.min_confidence {
            Language::Hebrew
        } else if keyword_ratio.1 > self.min_confidence {
            Language::Russian
        } else if keyword_ratio.0 > keyword_ratio.1 {
            Language::Hebrew
        } else if keyword_ratio.1 > keyword_ratio.0 {
            Language::Russian
        } else {
            Language::Unknown
        }
    }
} 