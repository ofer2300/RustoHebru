use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;
use crate::technical_terms::TechnicalTerm;
use whatlang::{Lang, Script};
use rust_stemmers::{Algorithm, Stemmer};

lazy_static! {
    static ref HE_WORD: Regex = Regex::new(r"[\u0590-\u05FF\u0483-\u0489]+").unwrap();
    static ref RU_WORD: Regex = Regex::new(r"[\u0400-\u04FF\u0483-\u0489]+").unwrap();
    static ref TERM_PATTERNS: HashMap<&'static str, Regex> = {
        let mut m = HashMap::new();
        // תבניות למונחים טכניים
        m.insert("standard", Regex::new(r"(?i)(ГОСТ|СНиП|СП|ТУ|ISO|תקן ישראלי|ת״י)\s*[-\d\.]+"").unwrap());
        m.insert("requirement", Regex::new(r"(?:דרישות|требования)\s+[\u0590-\u05FF\u0400-\u04FF]+").unwrap());
        m.insert("system", Regex::new(r"(?:מערכת|система)\s+[\u0590-\u05FF\u0400-\u04FF]+").unwrap());
        m.insert("test", Regex::new(r"(?:בדיקות|испытания)\s+[\u0590-\u05FF\u0400-\u04FF]+").unwrap());
        m
    };
}

pub struct TextAnalyzer {
    he_stemmer: Stemmer,
    ru_stemmer: Stemmer,
    context_cache: HashMap<String, String>,
    term_confidence: HashMap<String, f64>,
}

impl TextAnalyzer {
    pub fn new() -> Self {
        Self {
            he_stemmer: Stemmer::create(Algorithm::Hebrew),
            ru_stemmer: Stemmer::create(Algorithm::Russian),
            context_cache: HashMap::new(),
            term_confidence: HashMap::new(),
        }
    }

    pub fn analyze_text(&mut self, text: &str) -> Vec<TechnicalTerm> {
        let mut terms = Vec::new();
        let paragraphs = self.split_to_paragraphs(text);
        
        for paragraph in paragraphs {
            // זיהוי שפה עיקרית של הפסקה
            let lang = self.detect_main_language(&paragraph);
            
            // חילוץ מונחים לפי תבניות
            let paragraph_terms = self.extract_terms(&paragraph, lang);
            
            // הוספת הקשר לכל מונח
            for mut term in paragraph_terms {
                term.context = self.extract_context(&paragraph);
                term.confidence = self.calculate_confidence(&term, &paragraph);
                terms.push(term);
            }
        }

        // מיזוג מונחים דומים
        self.merge_similar_terms(&mut terms);
        
        terms
    }

    fn split_to_paragraphs(&self, text: &str) -> Vec<String> {
        text.split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .map(|p| p.trim().to_string())
            .collect()
    }

    fn detect_main_language(&self, text: &str) -> Lang {
        let info = whatlang::detect(text).unwrap_or_else(|| {
            whatlang::Detector::new()
                .with_whitelist(&[Lang::Heb, Lang::Rus])
                .detect(text)
                .unwrap_or_else(|| whatlang::Info::new("", Script::Unknown, Lang::Eng, 0.0))
        });
        info.lang()
    }

    fn extract_terms(&self, text: &str, main_lang: Lang) -> Vec<TechnicalTerm> {
        let mut terms = Vec::new();
        
        // חיפוש לפי תבניות מוגדרות מראש
        for (pattern_type, pattern) in TERM_PATTERNS.iter() {
            for cap in pattern.captures_iter(text) {
                if let Some(term) = self.process_capture(&cap[0], pattern_type, main_lang) {
                    terms.push(term);
                }
            }
        }

        // זיהוי צמדי מילים בשתי השפות
        let words: Vec<&str> = text.split_whitespace().collect();
        for window in words.windows(2) {
            if let [w1, w2] = window {
                if (HE_WORD.is_match(w1) && RU_WORD.is_match(w2)) ||
                   (RU_WORD.is_match(w1) && HE_WORD.is_match(w2)) {
                    if let Some(term) = self.create_term_from_pair(w1, w2) {
                        terms.push(term);
                    }
                }
            }
        }

        terms
    }

    fn process_capture(&self, text: &str, pattern_type: &str, main_lang: Lang) -> Option<TechnicalTerm> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let (term_he, term_ru) = match main_lang {
            Lang::Heb => (words[0].to_string(), words.get(1).map(|w| w.to_string())?),
            Lang::Rus => (words.get(1).map(|w| w.to_string())?, words[0].to_string()),
            _ => return None,
        };

        Some(TechnicalTerm {
            term_he,
            term_ru,
            domain: self.detect_domain(pattern_type, &text),
            context: String::new(), // יעודכן מאוחר יותר
            examples: self.generate_examples(&term_he, &term_ru),
            synonyms: self.find_synonyms(&term_ru),
            source: self.detect_source(&text),
            confidence: 0.0, // יעודכן מאוחר יותר
        })
    }

    fn create_term_from_pair(&self, w1: &str, w2: &str) -> Option<TechnicalTerm> {
        let (term_he, term_ru) = if HE_WORD.is_match(w1) {
            (w1.to_string(), w2.to_string())
        } else {
            (w2.to_string(), w1.to_string())
        };

        Some(TechnicalTerm {
            term_he,
            term_ru,
            domain: "general".to_string(),
            context: String::new(),
            examples: Vec::new(),
            synonyms: Vec::new(),
            source: String::new(),
            confidence: 0.0,
        })
    }

    fn extract_context(&mut self, text: &str) -> String {
        // בדיקה אם ההקשר כבר קיים במטמון
        if let Some(cached) = self.context_cache.get(text) {
            return cached.clone();
        }

        let context = if let Some(pos) = text.find('(') {
            if let Some(end) = text.find(')') {
                text[pos + 1..end].to_string()
            } else {
                String::new()
            }
        } else {
            // חיפוש הקשר לפי מילות מפתח
            let keywords = ["בהקשר של", "в контексте", "для", "עבור"];
            keywords.iter()
                .find_map(|&k| text.find(k).map(|i| {
                    let end = text[i..].find('.').unwrap_or(text.len());
                    text[i..i + end].trim().to_string()
                }))
                .unwrap_or_default()
        };

        // שמירה במטמון
        self.context_cache.insert(text.to_string(), context.clone());
        context
    }

    fn calculate_confidence(&mut self, term: &TechnicalTerm, context: &str) -> f64 {
        let mut confidence = 0.8; // בסיס

        // בדיקת מקור
        if !term.source.is_empty() {
            confidence += 0.1;
        }

        // בדיקת הקשר
        if !term.context.is_empty() {
            confidence += 0.05;
        }

        // בדיקת מילים נרדפות
        if !term.synonyms.is_empty() {
            confidence += 0.05;
        }

        // שמירת רמת הביטחון במטמון
        let key = format!("{}:{}", term.term_he, term.term_ru);
        self.term_confidence.insert(key, confidence);

        confidence.min(1.0)
    }

    fn merge_similar_terms(&self, terms: &mut Vec<TechnicalTerm>) {
        let mut i = 0;
        while i < terms.len() {
            let mut j = i + 1;
            while j < terms.len() {
                if self.are_terms_similar(&terms[i], &terms[j]) {
                    let merged = self.merge_terms(&terms[i], &terms[j]);
                    terms[i] = merged;
                    terms.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    fn are_terms_similar(&self, t1: &TechnicalTerm, t2: &TechnicalTerm) -> bool {
        let he_stem1 = self.he_stemmer.stem(&t1.term_he);
        let he_stem2 = self.he_stemmer.stem(&t2.term_he);
        let ru_stem1 = self.ru_stemmer.stem(&t1.term_ru);
        let ru_stem2 = self.ru_stemmer.stem(&t2.term_ru);

        he_stem1 == he_stem2 || ru_stem1 == ru_stem2
    }

    fn merge_terms(&self, t1: &TechnicalTerm, t2: &TechnicalTerm) -> TechnicalTerm {
        let mut merged = t1.clone();
        
        // שילוב מילים נרדפות
        merged.synonyms.extend(t2.synonyms.iter().cloned());
        merged.synonyms.dedup();

        // שילוב דוגמאות
        merged.examples.extend(t2.examples.iter().cloned());
        merged.examples.dedup();

        // בחירת המקור עם הביטחון הגבוה יותר
        if t2.confidence > t1.confidence {
            merged.source = t2.source.clone();
            merged.confidence = t2.confidence;
        }

        merged
    }

    fn detect_domain(&self, pattern_type: &str, text: &str) -> String {
        match pattern_type {
            "standard" => "standards",
            "requirement" => "requirements",
            "system" => "systems",
            "test" => "testing",
            _ => "general",
        }.to_string()
    }

    fn generate_examples(&self, term_he: &str, term_ru: &str) -> Vec<String> {
        let mut examples = Vec::new();
        
        // דוגמאות בעברית
        examples.push(format!("התקנת {}", term_he));
        examples.push(format!("בדיקת {}", term_he));
        
        // דוגמאות ברוסית מתורגמות
        examples.push(format!("установка {}", term_ru));
        examples.push(format!("проверка {}", term_ru));
        
        examples
    }

    fn find_synonyms(&self, term_ru: &str) -> Vec<String> {
        let mut synonyms = Vec::new();
        
        // קיצור אם המונח מורכב ממספר מילים
        if term_ru.contains(' ') {
            let abbrev: String = term_ru
                .split_whitespace()
                .map(|word| word.chars().next().unwrap_or(' '))
                .collect();
            synonyms.push(abbrev);
        }
        
        // הוספת וריאציות נפוצות
        if term_ru.ends_with("ция") {
            synonyms.push(term_ru.replace("ция", "ка"));
        }
        
        synonyms
    }

    fn detect_source(&self, text: &str) -> String {
        let source_patterns = [
            "ГОСТ", "СНиП", "СП", "ТУ", "ISO",
            "תקן ישראלי", "ת\"י", "מפרט כללי"
        ];
        
        for pattern in source_patterns.iter() {
            if let Some(pos) = text.find(pattern) {
                let end = text[pos..].find(|c| c == ' ' || c == '\n')
                    .unwrap_or(text[pos..].len());
                return text[pos..pos + end].to_string();
            }
        }
        
        String::new()
    }
} 