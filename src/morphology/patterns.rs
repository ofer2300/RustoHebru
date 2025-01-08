use super::*;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternRule {
    pub pattern: String,
    pub description: String,
    pub examples: Vec<String>,
    pub frequency: f32,
    pub variations: Vec<String>,
}

#[derive(Debug)]
pub struct PatternManager {
    hebrew_patterns: HashMap<String, Vec<PatternRule>>,
    pattern_index: HashMap<String, HashSet<String>>,
}

impl PatternManager {
    pub fn new() -> Self {
        Self {
            hebrew_patterns: Self::load_patterns(),
            pattern_index: HashMap::new(),
        }
    }

    fn load_patterns() -> HashMap<String, Vec<PatternRule>> {
        // כאן נטען את הדפוסים מקובץ JSON
        let mut patterns = HashMap::new();
        
        // דוגמה לדפוסים בסיסיים
        patterns.insert("פעל".to_string(), vec![
            PatternRule {
                pattern: "קטל".to_string(),
                description: "משקל פעל - פעולה בסיסית".to_string(),
                examples: vec!["כתב".to_string(), "למד".to_string(), "שמר".to_string()],
                frequency: 0.8,
                variations: vec!["קוטל".to_string(), "קטול".to_string()],
            }
        ]);
        
        patterns.insert("פיעל".to_string(), vec![
            PatternRule {
                pattern: "קיטל".to_string(),
                description: "משקל פיעל - פעולה אינטנסיבית".to_string(),
                examples: vec!["דיבר".to_string(), "שיחק".to_string(), "לימד".to_string()],
                frequency: 0.6,
                variations: vec!["מקטל".to_string(), "קיטול".to_string()],
            }
        ]);
        
        patterns
    }

    pub fn find_matching_patterns(&self, word: &str) -> Vec<PatternRule> {
        let mut matches = Vec::new();
        
        for (_, rules) in &self.hebrew_patterns {
            for rule in rules {
                if self.matches_pattern(word, &rule.pattern) {
                    matches.push(rule.clone());
                }
            }
        }
        
        // מיון לפי תדירות
        matches.sort_by(|a, b| b.frequency.partial_cmp(&a.frequency).unwrap());
        matches
    }

    fn matches_pattern(&self, word: &str, pattern: &str) -> bool {
        if word.len() != pattern.len() {
            return false;
        }

        let word_chars: Vec<char> = word.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        for (w, p) in word_chars.iter().zip(pattern_chars.iter()) {
            match p {
                'ק' | 'ט' | 'ל' => continue, // תווים שמייצגים שורש
                _ if w == p => continue,     // תווים זהים
                _ => return false,           // אי התאמה
            }
        }

        true
    }

    pub fn get_pattern_variations(&self, pattern: &str) -> Vec<String> {
        self.hebrew_patterns.get(pattern)
            .map(|rules| {
                rules.iter()
                    .flat_map(|rule| rule.variations.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_pattern_examples(&self, pattern: &str) -> Vec<String> {
        self.hebrew_patterns.get(pattern)
            .map(|rules| {
                rules.iter()
                    .flat_map(|rule| rule.examples.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn add_pattern(&mut self, category: String, rule: PatternRule) {
        self.hebrew_patterns
            .entry(category)
            .or_insert_with(Vec::new)
            .push(rule);
    }

    pub fn build_index(&mut self) {
        self.pattern_index.clear();
        
        for (category, rules) in &self.hebrew_patterns {
            for rule in rules {
                let key = rule.pattern.clone();
                self.pattern_index
                    .entry(key)
                    .or_insert_with(HashSet::new)
                    .insert(category.clone());
            }
        }
    }

    pub fn find_categories(&self, pattern: &str) -> HashSet<String> {
        self.pattern_index
            .get(pattern)
            .cloned()
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub text: String,
    pub category: PatternCategory,
    pub frequency: f32,
    pub confidence: f32,
    pub examples: Vec<String>,
    pub neural_features: Vec<f32>,
}

#[derive(Debug, Clone)]
pub enum PatternCategory {
    Verb {
        tense: String,
        person: String,
        number: String,
        gender: String,
    },
    Noun {
        number: String,
        gender: String,
        state: String,
    },
    Adjective {
        number: String,
        gender: String,
    },
    Adverb,
    Preposition,
    Other(String),
}

impl Pattern {
    pub fn new(text: String, category: PatternCategory) -> Self {
        Self {
            text,
            category,
            frequency: 0.0,
            confidence: 0.0,
            examples: Vec::new(),
            neural_features: Vec::new(),
        }
    }

    pub fn with_frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn add_example(&mut self, example: String) {
        self.examples.push(example);
    }

    pub fn set_neural_features(&mut self, features: Vec<f32>) {
        self.neural_features = features;
    }

    pub fn is_verb(&self) -> bool {
        matches!(self.category, PatternCategory::Verb { .. })
    }

    pub fn is_noun(&self) -> bool {
        matches!(self.category, PatternCategory::Noun { .. })
    }

    pub fn is_adjective(&self) -> bool {
        matches!(self.category, PatternCategory::Adjective { .. })
    }

    pub fn get_verb_info(&self) -> Option<(String, String, String, String)> {
        match &self.category {
            PatternCategory::Verb { tense, person, number, gender } => {
                Some((tense.clone(), person.clone(), number.clone(), gender.clone()))
            },
            _ => None,
        }
    }

    pub fn get_noun_info(&self) -> Option<(String, String, String)> {
        match &self.category {
            PatternCategory::Noun { number, gender, state } => {
                Some((number.clone(), gender.clone(), state.clone()))
            },
            _ => None,
        }
    }

    pub fn get_adjective_info(&self) -> Option<(String, String)> {
        match &self.category {
            PatternCategory::Adjective { number, gender } => {
                Some((number.clone(), gender.clone()))
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matching() {
        let manager = PatternManager::new();
        let matches = manager.find_matching_patterns("כתב");
        assert!(!matches.is_empty());
    }

    #[test]
    fn test_pattern_variations() {
        let manager = PatternManager::new();
        let variations = manager.get_pattern_variations("פעל");
        assert!(!variations.is_empty());
    }

    #[test]
    fn test_pattern_examples() {
        let manager = PatternManager::new();
        let examples = manager.get_pattern_examples("פעל");
        assert!(!examples.is_empty());
    }

    #[test]
    fn test_add_pattern() {
        let mut manager = PatternManager::new();
        let rule = PatternRule {
            pattern: "הקטל".to_string(),
            description: "משקל הפעיל".to_string(),
            examples: vec!["הכתיב".to_string()],
            frequency: 0.5,
            variations: vec!["מקטיל".to_string()],
        };
        
        manager.add_pattern("הפעיל".to_string(), rule);
        let matches = manager.find_matching_patterns("הכתיב");
        assert!(!matches.is_empty());
    }
} 