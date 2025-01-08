use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationMetrics {
    pub bleu_score: f64,
    pub meteor_score: f64,
    pub ter_score: f64,
    pub chrf_score: f64,
    pub technical_accuracy: f64,
    pub fluency_score: f64,
    pub adequacy_score: f64,
    pub error_analysis: ErrorAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorAnalysis {
    pub technical_terms: Vec<TermError>,
    pub grammar_errors: Vec<GrammarError>,
    pub style_errors: Vec<StyleError>,
    pub context_errors: Vec<ContextError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermError {
    pub source_term: String,
    pub translated_term: String,
    pub expected_term: String,
    pub context: String,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarError {
    pub error_type: GrammarErrorType,
    pub text_span: String,
    pub suggestion: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrammarErrorType {
    Agreement,
    Tense,
    Number,
    Gender,
    Case,
    Word_Order,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleError {
    pub error_type: StyleErrorType,
    pub text_span: String,
    pub suggestion: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StyleErrorType {
    Formality,
    Register,
    Consistency,
    Clarity,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextError {
    pub error_type: ContextErrorType,
    pub text_span: String,
    pub context: String,
    pub suggestion: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextErrorType {
    Domain,
    Cultural,
    Pragmatic,
    Other(String),
}

pub struct Evaluator {
    reference_translations: HashMap<String, String>,
    technical_terms: HashMap<String, String>,
    style_guide: StyleGuide,
}

impl Evaluator {
    pub fn new(
        reference_translations: HashMap<String, String>,
        technical_terms: HashMap<String, String>,
        style_guide: StyleGuide,
    ) -> Self {
        Self {
            reference_translations,
            technical_terms,
            style_guide,
        }
    }
    
    pub fn evaluate(&self, source_text: &str, translated_text: &str) -> EvaluationMetrics {
        let reference = self.reference_translations.get(source_text)
            .map(|s| s.as_str())
            .unwrap_or("");
            
        EvaluationMetrics {
            bleu_score: self.calculate_bleu(translated_text, reference),
            meteor_score: self.calculate_meteor(translated_text, reference),
            ter_score: self.calculate_ter(translated_text, reference),
            chrf_score: self.calculate_chrf(translated_text, reference),
            technical_accuracy: self.evaluate_technical_accuracy(source_text, translated_text),
            fluency_score: self.evaluate_fluency(translated_text),
            adequacy_score: self.evaluate_adequacy(source_text, translated_text),
            error_analysis: self.analyze_errors(source_text, translated_text),
        }
    }
    
    fn calculate_bleu(&self, hypothesis: &str, reference: &str) -> f64 {
        // חישוב ציון BLEU
        let hyp_words: Vec<&str> = hypothesis.split_whitespace().collect();
        let ref_words: Vec<&str> = reference.split_whitespace().collect();
        
        let mut matches = 0;
        let mut total = 0;
        
        // חישוב התאמות ברמת המילה
        for window_size in 1..=4 {
            for i in 0..=hyp_words.len().saturating_sub(window_size) {
                let hyp_ngram: Vec<_> = hyp_words[i..i+window_size].to_vec();
                
                for j in 0..=ref_words.len().saturating_sub(window_size) {
                    let ref_ngram: Vec<_> = ref_words[j..j+window_size].to_vec();
                    
                    if hyp_ngram == ref_ngram {
                        matches += 1;
                        break;
                    }
                }
                
                total += 1;
            }
        }
        
        if total == 0 {
            0.0
        } else {
            matches as f64 / total as f64
        }
    }
    
    fn calculate_meteor(&self, hypothesis: &str, reference: &str) -> f64 {
        // חישוב ציון METEOR
        let hyp_words: Vec<&str> = hypothesis.split_whitespace().collect();
        let ref_words: Vec<&str> = reference.split_whitespace().collect();
        
        let mut matches = 0;
        let mut chunks = 0;
        let mut previous_match = false;
        
        for (i, hyp_word) in hyp_words.iter().enumerate() {
            for (j, ref_word) in ref_words.iter().enumerate() {
                if hyp_word == ref_word {
                    matches += 1;
                    if !previous_match {
                        chunks += 1;
                    }
                    previous_match = true;
                    break;
                }
            }
            if !previous_match {
                previous_match = false;
            }
        }
        
        let precision = if hyp_words.is_empty() { 0.0 } else { matches as f64 / hyp_words.len() as f64 };
        let recall = if ref_words.is_empty() { 0.0 } else { matches as f64 / ref_words.len() as f64 };
        
        if precision == 0.0 || recall == 0.0 {
            0.0
        } else {
            let fmean = 10.0 * precision * recall / (9.0 * precision + recall);
            let penalty = 0.5 * (chunks as f64 / matches as f64).powf(3.0);
            fmean * (1.0 - penalty)
        }
    }
    
    fn calculate_ter(&self, hypothesis: &str, reference: &str) -> f64 {
        // חישוב ציון TER (Translation Edit Rate)
        let hyp_words: Vec<&str> = hypothesis.split_whitespace().collect();
        let ref_words: Vec<&str> = reference.split_whitespace().collect();
        
        let mut dp = vec![vec![0; ref_words.len() + 1]; hyp_words.len() + 1];
        
        // אתחול מטריצת המרחקים
        for i in 0..=hyp_words.len() {
            dp[i][0] = i;
        }
        for j in 0..=ref_words.len() {
            dp[0][j] = j;
        }
        
        // חישוב מרחק עריכה מינימלי
        for i in 1..=hyp_words.len() {
            for j in 1..=ref_words.len() {
                let substitution_cost = if hyp_words[i-1] == ref_words[j-1] { 0 } else { 1 };
                
                dp[i][j] = (dp[i-1][j] + 1) // מחיקה
                    .min(dp[i][j-1] + 1) // הוספה
                    .min(dp[i-1][j-1] + substitution_cost); // החלפה
            }
        }
        
        let edit_distance = dp[hyp_words.len()][ref_words.len()];
        edit_distance as f64 / ref_words.len() as f64
    }
    
    fn calculate_chrf(&self, hypothesis: &str, reference: &str) -> f64 {
        // חישוב ציון chrF (Character n-gram F-score)
        let mut char_matches = 0;
        let mut total_chars = 0;
        
        // חישוב התאמות ברמת התווים
        for window_size in 1..=6 {
            let hyp_chars: Vec<char> = hypothesis.chars().collect();
            let ref_chars: Vec<char> = reference.chars().collect();
            
            for i in 0..=hyp_chars.len().saturating_sub(window_size) {
                let hyp_ngram: Vec<_> = hyp_chars[i..i+window_size].to_vec();
                
                for j in 0..=ref_chars.len().saturating_sub(window_size) {
                    let ref_ngram: Vec<_> = ref_chars[j..j+window_size].to_vec();
                    
                    if hyp_ngram == ref_ngram {
                        char_matches += 1;
                        break;
                    }
                }
                
                total_chars += 1;
            }
        }
        
        if total_chars == 0 {
            0.0
        } else {
            char_matches as f64 / total_chars as f64
        }
    }
    
    fn evaluate_technical_accuracy(&self, source_text: &str, translated_text: &str) -> f64 {
        let mut correct_terms = 0;
        let mut total_terms = 0;
        
        // בדיקת תרגום מונחים טכניים
        for (source_term, expected_translation) in &self.technical_terms {
            if source_text.contains(source_term) {
                total_terms += 1;
                if translated_text.contains(expected_translation) {
                    correct_terms += 1;
                }
            }
        }
        
        if total_terms == 0 {
            1.0
        } else {
            correct_terms as f64 / total_terms as f64
        }
    }
    
    fn evaluate_fluency(&self, translated_text: &str) -> f64 {
        // הערכת שטף הטקסט המתורגם
        let mut score = 1.0;
        
        // בדיקת שגיאות דקדוק
        let grammar_errors = self.check_grammar(translated_text);
        score -= 0.1 * grammar_errors.len() as f64;
        
        // בדיקת עקביות סגנונית
        let style_errors = self.check_style(translated_text);
        score -= 0.1 * style_errors.len() as f64;
        
        score.max(0.0)
    }
    
    fn evaluate_adequacy(&self, source_text: &str, translated_text: &str) -> f64 {
        // הערכת דיוק התרגום
        let mut score = 1.0;
        
        // בדיקת שלמות המידע
        let missing_info = self.check_missing_information(source_text, translated_text);
        score -= 0.2 * missing_info as f64;
        
        // בדיקת דיוק הקשרי
        let context_errors = self.check_context(source_text, translated_text);
        score -= 0.1 * context_errors.len() as f64;
        
        score.max(0.0)
    }
    
    fn analyze_errors(&self, source_text: &str, translated_text: &str) -> ErrorAnalysis {
        ErrorAnalysis {
            technical_terms: self.analyze_term_errors(source_text, translated_text),
            grammar_errors: self.check_grammar(translated_text),
            style_errors: self.check_style(translated_text),
            context_errors: self.check_context(source_text, translated_text),
        }
    }
    
    fn analyze_term_errors(&self, source_text: &str, translated_text: &str) -> Vec<TermError> {
        let mut errors = Vec::new();
        
        for (source_term, expected_translation) in &self.technical_terms {
            if source_text.contains(source_term) && !translated_text.contains(expected_translation) {
                errors.push(TermError {
                    source_term: source_term.clone(),
                    translated_term: "".to_string(), // יש להוסיף זיהוי של התרגום בפועל
                    expected_term: expected_translation.clone(),
                    context: "".to_string(), // יש להוסיף הקשר
                    domain: "".to_string(), // יש להוסיף תחום
                });
            }
        }
        
        errors
    }
    
    fn check_grammar(&self, text: &str) -> Vec<GrammarError> {
        let mut errors = Vec::new();
        
        // בדיקת התאמה במין ומספר
        // TODO: להוסיף בדיקות דקדוק מפורטות
        
        errors
    }
    
    fn check_style(&self, text: &str) -> Vec<StyleError> {
        let mut errors = Vec::new();
        
        // בדיקת התאמה לסגנון הנדרש
        if !self.style_guide.check_formality(text) {
            errors.push(StyleError {
                error_type: StyleErrorType::Formality,
                text_span: text.to_string(),
                suggestion: "".to_string(),
                explanation: "רמת הפורמליות אינה מתאימה".to_string(),
            });
        }
        
        errors
    }
    
    fn check_context(&self, source_text: &str, translated_text: &str) -> Vec<ContextError> {
        let mut errors = Vec::new();
        
        // בדיקת התאמה להקשר
        // TODO: להוסיף בדיקות הקשר מפורטות
        
        errors
    }
    
    fn check_missing_information(&self, source_text: &str, translated_text: &str) -> usize {
        // בדיקת מידע חסר
        let source_sentences: Vec<&str> = source_text.split(['.', '!', '?']).collect();
        let translated_sentences: Vec<&str> = translated_text.split(['.', '!', '?']).collect();
        
        source_sentences.len().saturating_sub(translated_sentences.len())
    }
}

#[derive(Debug, Clone)]
pub struct StyleGuide {
    formality_level: FormalityLevel,
    domain_specific_rules: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub enum FormalityLevel {
    Formal,
    SemiFormal,
    Informal,
}

impl StyleGuide {
    pub fn new(formality_level: FormalityLevel) -> Self {
        Self {
            formality_level,
            domain_specific_rules: HashMap::new(),
        }
    }
    
    pub fn add_domain_rule(&mut self, domain: String, rules: Vec<String>) {
        self.domain_specific_rules.insert(domain, rules);
    }
    
    fn check_formality(&self, text: &str) -> bool {
        // בדיקת התאמה לרמת הפורמליות
        match self.formality_level {
            FormalityLevel::Formal => !text.contains("אתה") && !text.contains("את"),
            FormalityLevel::SemiFormal => true,
            FormalityLevel::Informal => true,
        }
    }
} 