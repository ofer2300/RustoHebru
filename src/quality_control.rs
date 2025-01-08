use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;
use crate::morphology::{
    HebrewMorphology, RussianMorphology,
    HebrewAnalyzer, RussianAnalyzer,
    Gender, Number, Case,
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::evaluation::{EvaluationMetrics, ErrorAnalysis};
use crate::technical_dictionary::TechnicalDictionary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityReport {
    pub source_text: String,
    pub translated_text: String,
    pub metrics: EvaluationMetrics,
    pub validation_results: ValidationResults,
    pub suggestions: Vec<Suggestion>,
    pub overall_quality_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResults {
    pub technical_terms: Vec<TermValidation>,
    pub grammar: Vec<GrammarValidation>,
    pub style: Vec<StyleValidation>,
    pub context: Vec<ContextValidation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermValidation {
    pub term: String,
    pub translation: String,
    pub expected: String,
    pub is_valid: bool,
    pub context: String,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarValidation {
    pub text: String,
    pub error_type: String,
    pub suggestion: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleValidation {
    pub text: String,
    pub issue: String,
    pub expected_style: String,
    pub suggestion: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextValidation {
    pub text: String,
    pub context_type: String,
    pub issue: String,
    pub suggestion: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub text: String,
    pub suggestion: String,
    pub reason: String,
    pub category: SuggestionCategory,
    pub priority: Priority,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Critical,
    Major,
    Minor,
    Info,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SuggestionCategory {
    Technical,
    Grammar,
    Style,
    Context,
    General,
}

pub struct QualityController {
    technical_dictionary: TechnicalDictionary,
    grammar_rules: Vec<GrammarRule>,
    style_rules: Vec<StyleRule>,
    context_rules: Vec<ContextRule>,
    quality_thresholds: QualityThresholds,
}

impl QualityController {
    pub fn new(
        technical_dictionary: TechnicalDictionary,
        quality_thresholds: Option<QualityThresholds>,
    ) -> Self {
        Self {
            technical_dictionary,
            grammar_rules: Self::load_grammar_rules(),
            style_rules: Self::load_style_rules(),
            context_rules: Self::load_context_rules(),
            quality_thresholds: quality_thresholds.unwrap_or_default(),
        }
    }
    
    pub fn check_quality(&self, source_text: &str, translated_text: &str) -> QualityReport {
        // בדיקת מונחים טכניים
        let term_validations = self.validate_technical_terms(source_text, translated_text);
        
        // בדיקות דקדוק
        let grammar_validations = self.validate_grammar(translated_text);
        
        // בדיקות סגנון
        let style_validations = self.validate_style(translated_text);
        
        // בדיקות הקשר
        let context_validations = self.validate_context(source_text, translated_text);
        
        // חישוב מדדי איכות
        let metrics = self.calculate_metrics(
            source_text,
            translated_text,
            &term_validations,
            &grammar_validations,
            &style_validations,
            &context_validations,
        );
        
        // יצירת הצעות לשיפור
        let suggestions = self.generate_suggestions(
            &term_validations,
            &grammar_validations,
            &style_validations,
            &context_validations,
        );
        
        // חישוב ציון איכות כולל
        let overall_quality_score = self.calculate_overall_score(&metrics, &term_validations);
        
        QualityReport {
            source_text: source_text.to_string(),
            translated_text: translated_text.to_string(),
            metrics,
            validation_results: ValidationResults {
                technical_terms: term_validations,
                grammar: grammar_validations,
                style: style_validations,
                context: context_validations,
            },
            suggestions,
            overall_quality_score,
        }
    }
    
    fn validate_technical_terms(&self, source_text: &str, translated_text: &str) -> Vec<TermValidation> {
        let mut validations = Vec::new();
        
        // בדיקת מונחים טכניים
        for (term, expected) in self.technical_dictionary.get_terms() {
            if source_text.contains(term) {
                validations.push(TermValidation {
                    term: term.to_string(),
                    translation: self.find_term_translation(term, translated_text),
                    expected: expected.to_string(),
                    is_valid: translated_text.contains(&expected),
                    context: self.technical_dictionary.get_context(term)
                        .unwrap_or_default(),
                    domain: self.technical_dictionary.get_domain(term)
                        .unwrap_or_default(),
                });
            }
        }
        
        validations
    }
    
    fn validate_grammar(&self, text: &str) -> Vec<GrammarValidation> {
        let mut validations = Vec::new();
        
        for rule in &self.grammar_rules {
            if let Some(error) = rule.check(text) {
                validations.push(GrammarValidation {
                    text: error.text.to_string(),
                    error_type: error.error_type,
                    suggestion: error.suggestion,
                    severity: error.severity,
                });
            }
        }
        
        validations
    }
    
    fn validate_style(&self, text: &str) -> Vec<StyleValidation> {
        let mut validations = Vec::new();
        
        for rule in &self.style_rules {
            if let Some(error) = rule.check(text) {
                validations.push(StyleValidation {
                    text: error.text.to_string(),
                    issue: error.issue,
                    expected_style: error.expected_style,
                    suggestion: error.suggestion,
                    severity: error.severity,
                });
            }
        }
        
        validations
    }
    
    fn validate_context(&self, source_text: &str, translated_text: &str) -> Vec<ContextValidation> {
        let mut validations = Vec::new();
        
        for rule in &self.context_rules {
            if let Some(error) = rule.check(source_text, translated_text) {
                validations.push(ContextValidation {
                    text: error.text.to_string(),
                    context_type: error.context_type,
                    issue: error.issue,
                    suggestion: error.suggestion,
                    severity: error.severity,
                });
            }
        }
        
        validations
    }
    
    fn calculate_metrics(
        &self,
        source_text: &str,
        translated_text: &str,
        term_validations: &[TermValidation],
        grammar_validations: &[GrammarValidation],
        style_validations: &[StyleValidation],
        context_validations: &[ContextValidation],
    ) -> EvaluationMetrics {
        // חישוב מדדי איכות שונים
        let technical_accuracy = self.calculate_technical_accuracy(term_validations);
        let fluency_score = self.calculate_fluency_score(grammar_validations, style_validations);
        let adequacy_score = self.calculate_adequacy_score(context_validations);
        
        EvaluationMetrics {
            bleu_score: 0.0, // יש להוסיף חישוב BLEU
            meteor_score: 0.0, // יש להוסיף חישוב METEOR
            ter_score: 0.0, // יש להוסיף חישוב TER
            chrf_score: 0.0, // יש להוסיף חישוב chrF
            technical_accuracy,
            fluency_score,
            adequacy_score,
            error_analysis: ErrorAnalysis::default(),
        }
    }
    
    fn generate_suggestions(
        &self,
        term_validations: &[TermValidation],
        grammar_validations: &[GrammarValidation],
        style_validations: &[StyleValidation],
        context_validations: &[ContextValidation],
    ) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        
        // הצעות למונחים טכניים
        for validation in term_validations {
            if !validation.is_valid {
                suggestions.push(Suggestion {
                    text: validation.translation.clone(),
                    suggestion: validation.expected.clone(),
                    reason: format!("מונח טכני לא תקין בתחום {}", validation.domain),
                    category: SuggestionCategory::Technical,
                    priority: Priority::High,
                });
            }
        }
        
        // הצעות לתיקוני דקדוק
        for validation in grammar_validations {
            suggestions.push(Suggestion {
                text: validation.text.clone(),
                suggestion: validation.suggestion.clone(),
                reason: validation.error_type.clone(),
                category: SuggestionCategory::Grammar,
                priority: match validation.severity {
                    Severity::Critical => Priority::High,
                    Severity::Major => Priority::High,
                    Severity::Minor => Priority::Medium,
                    Severity::Info => Priority::Low,
                },
            });
        }
        
        // הצעות לשיפור סגנון
        for validation in style_validations {
            suggestions.push(Suggestion {
                text: validation.text.clone(),
                suggestion: validation.suggestion.clone(),
                reason: validation.issue.clone(),
                category: SuggestionCategory::Style,
                priority: match validation.severity {
                    Severity::Critical => Priority::High,
                    Severity::Major => Priority::Medium,
                    Severity::Minor => Priority::Low,
                    Severity::Info => Priority::Low,
                },
            });
        }
        
        // הצעות להקשר
        for validation in context_validations {
            suggestions.push(Suggestion {
                text: validation.text.clone(),
                suggestion: validation.suggestion.clone(),
                reason: validation.issue.clone(),
                category: SuggestionCategory::Context,
                priority: match validation.severity {
                    Severity::Critical => Priority::High,
                    Severity::Major => Priority::Medium,
                    Severity::Minor => Priority::Low,
                    Severity::Info => Priority::Low,
                },
            });
        }
        
        // מיון ההצעות לפי עדיפות
        suggestions.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        suggestions
    }
    
    fn calculate_overall_score(&self, metrics: &EvaluationMetrics, term_validations: &[TermValidation]) -> f64 {
        let technical_weight = 0.4;
        let fluency_weight = 0.3;
        let adequacy_weight = 0.3;
        
        let technical_score = metrics.technical_accuracy;
        let fluency_score = metrics.fluency_score;
        let adequacy_score = metrics.adequacy_score;
        
        technical_weight * technical_score +
            fluency_weight * fluency_score +
            adequacy_weight * adequacy_score
    }
    
    fn calculate_technical_accuracy(&self, validations: &[TermValidation]) -> f64 {
        if validations.is_empty() {
            return 1.0;
        }
        
        let valid_terms = validations.iter()
            .filter(|v| v.is_valid)
            .count();
            
        valid_terms as f64 / validations.len() as f64
    }
    
    fn calculate_fluency_score(
        &self,
        grammar_validations: &[GrammarValidation],
        style_validations: &[StyleValidation],
    ) -> f64 {
        let mut score = 1.0;
        
        // הורדת ניקוד עבור שגיאות דקדוק
        for validation in grammar_validations {
            score -= match validation.severity {
                Severity::Critical => 0.2,
                Severity::Major => 0.1,
                Severity::Minor => 0.05,
                Severity::Info => 0.01,
            };
        }
        
        // הורדת ניקוד עבור בעיות סגנון
        for validation in style_validations {
            score -= match validation.severity {
                Severity::Critical => 0.15,
                Severity::Major => 0.08,
                Severity::Minor => 0.03,
                Severity::Info => 0.01,
            };
        }
        
        score.max(0.0)
    }
    
    fn calculate_adequacy_score(&self, context_validations: &[ContextValidation]) -> f64 {
        let mut score = 1.0;
        
        // הורדת ניקוד עבור בעיות הקשר
        for validation in context_validations {
            score -= match validation.severity {
                Severity::Critical => 0.2,
                Severity::Major => 0.1,
                Severity::Minor => 0.05,
                Severity::Info => 0.01,
            };
        }
        
        score.max(0.0)
    }
    
    fn find_term_translation(&self, term: &str, text: &str) -> String {
        // TODO: מימוש מתקדם יותר למציאת התרגום בפועל
        text.to_string()
    }
    
    fn load_grammar_rules() -> Vec<GrammarRule> {
        // טעינת חוקי דקדוק
        vec![]
    }
    
    fn load_style_rules() -> Vec<StyleRule> {
        // טעינת חוקי סגנון
        vec![]
    }
    
    fn load_context_rules() -> Vec<ContextRule> {
        // טעינת חוקי הקשר
        vec![]
    }
}

#[derive(Debug, Clone)]
struct GrammarRule {
    pattern: String,
    error_type: String,
    suggestion: String,
    severity: Severity,
}

impl GrammarRule {
    fn check(&self, text: &str) -> Option<GrammarError> {
        // TODO: מימוש בדיקת חוקי דקדוק
        None
    }
}

#[derive(Debug, Clone)]
struct GrammarError {
    text: String,
    error_type: String,
    suggestion: String,
    severity: Severity,
}

#[derive(Debug, Clone)]
struct StyleRule {
    pattern: String,
    expected_style: String,
    suggestion: String,
    severity: Severity,
}

impl StyleRule {
    fn check(&self, text: &str) -> Option<StyleError> {
        // TODO: מימוש בדיקת חוקי סגנון
        None
    }
}

#[derive(Debug, Clone)]
struct StyleError {
    text: String,
    issue: String,
    expected_style: String,
    suggestion: String,
    severity: Severity,
}

#[derive(Debug, Clone)]
struct ContextRule {
    pattern: String,
    context_type: String,
    suggestion: String,
    severity: Severity,
}

impl ContextRule {
    fn check(&self, source_text: &str, translated_text: &str) -> Option<ContextError> {
        // TODO: מימוש בדיקת חוקי הקשר
        None
    }
}

#[derive(Debug, Clone)]
struct ContextError {
    text: String,
    context_type: String,
    issue: String,
    suggestion: String,
    severity: Severity,
}

#[derive(Debug, Clone)]
pub struct QualityThresholds {
    pub min_technical_accuracy: f64,
    pub min_fluency_score: f64,
    pub min_adequacy_score: f64,
    pub min_overall_score: f64,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            min_technical_accuracy: 0.95,
            min_fluency_score: 0.8,
            min_adequacy_score: 0.8,
            min_overall_score: 0.85,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_technical_terms_validation() {
        let mut dictionary = TechnicalDictionary::new();
        dictionary.add_term("sprinkler", "ספרינקלר", "fire_protection", "");
        
        let controller = QualityController::new(dictionary, None);
        
        let report = controller.check_quality(
            "Install a sprinkler system",
            "התקן מערכת ספרינקלר",
        );
        
        assert!(report.validation_results.technical_terms[0].is_valid);
        assert!(report.overall_quality_score > 0.9);
    }
    
    #[test]
    fn test_invalid_translation() {
        let mut dictionary = TechnicalDictionary::new();
        dictionary.add_term("sprinkler", "ספרינקלר", "fire_protection", "");
        
        let controller = QualityController::new(dictionary, None);
        
        let report = controller.check_quality(
            "Install a sprinkler system",
            "התקן מערכת כיבוי",
        );
        
        assert!(!report.validation_results.technical_terms[0].is_valid);
        assert!(report.overall_quality_score < 0.9);
    }
} 