use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalTerm {
    pub term_he: String,
    pub term_ru: String,
    pub domain: String,
    pub context: String,
    pub examples: Vec<String>,
    pub synonyms: Vec<String>,
    pub source: String,
    pub confidence: f64,
}

#[derive(Debug)]
pub struct TermsDatabase {
    terms: HashMap<String, TechnicalTerm>,
    domain_patterns: HashMap<String, Vec<Regex>>,
    context_keywords: HashMap<String, Vec<String>>,
}

impl TermsDatabase {
    pub fn new() -> Self {
        Self {
            terms: HashMap::new(),
            domain_patterns: Self::init_domain_patterns(),
            context_keywords: Self::init_context_keywords(),
        }
    }

    pub fn add_term(&mut self, term: TechnicalTerm) {
        self.terms.insert(term.term_he.clone(), term.clone());
        // הוספת המונח גם במפתח רוסי
        self.terms.insert(term.term_ru.clone(), term);
    }

    pub fn get_term(&self, term: &str) -> Option<&TechnicalTerm> {
        self.terms.get(term)
    }

    pub fn find_terms_in_text(&self, text: &str) -> Vec<TechnicalTerm> {
        let mut found_terms = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();

        // חיפוש מונחים בודדים
        for word in &words {
            if let Some(term) = self.get_term(word) {
                found_terms.push(term.clone());
            }
        }

        // חיפוש ביטויים מרובי מילים
        for window_size in 2..=5 {
            for window in words.windows(window_size) {
                let phrase = window.join(" ");
                if let Some(term) = self.get_term(&phrase) {
                    found_terms.push(term.clone());
                }
            }
        }

        // זיהוי מונחים לפי הקשר
        for (domain, patterns) in &self.domain_patterns {
            for pattern in patterns {
                if pattern.is_match(text) {
                    // חיפוש מונחים נוספים מאותו תחום
                    for term in self.terms.values() {
                        if term.domain == *domain && !found_terms.contains(term) {
                            found_terms.push(term.clone());
                        }
                    }
                }
            }
        }

        found_terms
    }

    pub fn suggest_translations(&self, term: &str, context: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // חיפוש תרגום ישיר
        if let Some(term_data) = self.get_term(term) {
            suggestions.push(term_data.term_ru.clone());
            suggestions.extend(term_data.synonyms.clone());
        }

        // חיפוש מונחים דומים באותו הקשר
        let domain = self.detect_domain(context);
        for term_data in self.terms.values() {
            if term_data.domain == domain && !suggestions.contains(&term_data.term_ru) {
                suggestions.push(term_data.term_ru.clone());
            }
        }

        suggestions
    }

    fn detect_domain(&self, text: &str) -> String {
        let mut domain_scores = HashMap::new();

        // בדיקת מילות מפתח
        for (domain, keywords) in &self.context_keywords {
            let mut score = 0;
            for keyword in keywords {
                if text.contains(keyword) {
                    score += 1;
                }
            }
            domain_scores.insert(domain, score);
        }

        // בדיקת תבניות
        for (domain, patterns) in &self.domain_patterns {
            let score = patterns.iter()
                .filter(|pattern| pattern.is_match(text))
                .count();
            *domain_scores.entry(domain).or_insert(0) += score;
        }

        // בחירת התחום עם הניקוד הגבוה ביותר
        domain_scores.into_iter()
            .max_by_key(|&(_, score)| score)
            .map(|(domain, _)| domain.clone())
            .unwrap_or_else(|| "general".to_string())
    }

    fn init_domain_patterns() -> HashMap<String, Vec<Regex>> {
        let mut patterns = HashMap::new();
        
        // תבניות לזיהוי מונחים בתחום האינסטלציה
        patterns.insert(
            "plumbing".to_string(),
            vec![
                Regex::new(r"צינור\s+\d+").unwrap(),
                Regex::new(r"ברז\s+[א-ת]+").unwrap(),
                Regex::new(r"מערכת\s+אספקת\s+מים").unwrap(),
            ]
        );

        // תבניות לזיהוי מונחים בתחום כיבוי אש
        patterns.insert(
            "fire_safety".to_string(),
            vec![
                Regex::new(r"מערכת\s+כיבוי").unwrap(),
                Regex::new(r"גלאי\s+[א-ת]+").unwrap(),
                Regex::new(r"ספרינקלר\s+\d+").unwrap(),
            ]
        );

        // תבניות לזיהוי מונחים בתחום החשמל
        patterns.insert(
            "electrical".to_string(),
            vec![
                Regex::new(r"מעגל\s+חשמלי").unwrap(),
                Regex::new(r"הארקה\s+[א-ת]+").unwrap(),
                Regex::new(r"מתח\s+\d+").unwrap(),
            ]
        );

        patterns
    }

    fn init_context_keywords() -> HashMap<String, Vec<String>> {
        let mut keywords = HashMap::new();
        
        // מילות מפתח לתחום האינסטלציה
        keywords.insert(
            "plumbing".to_string(),
            vec![
                "צינור".to_string(),
                "ברז".to_string(),
                "מים".to_string(),
                "ניקוז".to_string(),
                "שסתום".to_string(),
            ]
        );

        // מילות מפתח לתחום כיבוי אש
        keywords.insert(
            "fire_safety".to_string(),
            vec![
                "כיבוי".to_string(),
                "אש".to_string(),
                "גלאי".to_string(),
                "ספרינקלר".to_string(),
                "חירום".to_string(),
            ]
        );

        // מילות מפתח לתחום החשמל
        keywords.insert(
            "electrical".to_string(),
            vec![
                "חשמל".to_string(),
                "מתח".to_string(),
                "זרם".to_string(),
                "הארקה".to_string(),
                "מעגל".to_string(),
            ]
        );

        keywords
    }

    pub fn validate_translation(&self, source_text: &str, translated_text: &str) -> bool {
        let source_terms = self.find_terms_in_text(source_text);
        let translated_terms = self.find_terms_in_text(translated_text);

        for source_term in &source_terms {
            let mut found_match = false;
            for translated_term in &translated_terms {
                if source_term.term_ru == translated_term.term_he || 
                   source_term.term_he == translated_term.term_ru {
                    found_match = true;
                    break;
                }
            }
            if !found_match {
                return false;
            }
        }

        true
    }

    pub fn get_term_info(&self, term: &str) -> Option<String> {
        self.get_term(term).map(|term_data| {
            format!(
                "מונח: {}\nתרגום: {}\nתחום: {}\nהקשר: {}\nדוגמאות: {}\nמילים נרדפות: {}\nמקור: {}\nרמת ביטחון: {:.2}",
                term_data.term_he,
                term_data.term_ru,
                term_data.domain,
                term_data.context,
                term_data.examples.join(", "),
                term_data.synonyms.join(", "),
                term_data.source,
                term_data.confidence
            )
        })
    }
} 