use std::collections::HashMap;
use std::sync::Arc;
use std::fmt;
use serde::{Serialize, Deserialize};
use tch::{nn, Tensor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tense {
    Past,
    Present,
    Future,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Aspect {
    Perfective,
    Imperfective,
}

#[derive(Debug, Clone)]
pub struct AnalysisContext {
    pub text: String,
    pub language: String,
    pub domain: String,
    pub register: String,
    pub style_level: String,
    pub semantic_field: String,
    pub neural_context: Vec<f32>,
    pub confidence: f32,
}

impl AnalysisContext {
    pub fn new(
        text: String,
        language: String,
        domain: String,
        register: String,
        style_level: String,
        semantic_field: String,
    ) -> Self {
        Self {
            text,
            language,
            domain,
            register,
            style_level,
            semantic_field,
            neural_context: Vec::new(),
            confidence: 0.0,
        }
    }

    pub fn with_neural_context(mut self, context: Vec<f32>) -> Self {
        self.neural_context = context;
        self
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn update_neural_context(&mut self, context: Vec<f32>) {
        self.neural_context = context;
    }

    pub fn update_confidence(&mut self, confidence: f32) {
        self.confidence = confidence;
    }

    pub fn is_formal(&self) -> bool {
        self.register.to_lowercase() == "formal"
    }

    pub fn is_informal(&self) -> bool {
        self.register.to_lowercase() == "informal"
    }

    pub fn is_high_style(&self) -> bool {
        self.style_level.to_lowercase() == "high"
    }

    pub fn is_low_style(&self) -> bool {
        self.style_level.to_lowercase() == "low"
    }

    pub fn matches_domain(&self, domain: &str) -> bool {
        self.domain.to_lowercase() == domain.to_lowercase()
    }

    pub fn matches_semantic_field(&self, field: &str) -> bool {
        self.semantic_field.to_lowercase() == field.to_lowercase()
    }
}

#[derive(Debug)]
pub enum MorphologyError {
    AnalysisError(String),
    NetworkError(String),
    CacheError(String),
}

pub trait MorphologyAnalyzer {
    type Analysis;
    
    fn analyze(&self, text: &str) -> Result<Self::Analysis, MorphologyError>;
    fn calculate_confidence(&self, features: &[f32]) -> f32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gender {
    Masculine,
    Feminine,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Number {
    Singular,
    Plural,
    Dual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphologyAnalysis {
    pub base_form: String,
    pub gender: Option<Gender>,
    pub number: Option<Number>,
    pub confidence: f32,
}

pub mod hebrew;
pub mod russian;
pub mod cache;
pub mod patterns;
pub mod semantic;
pub mod statistics;

pub use hebrew::HebrewAnalyzer;
pub use russian::RussianAnalyzer;
pub use cache::MorphologyCache;

#[derive(Debug, Clone)]
pub struct AnalyzerConfig {
    pub min_confidence: f32,
    pub max_candidates: usize,
    pub use_cache: bool,
    pub cache_size: usize,
    pub enable_neural: bool,
    pub neural_threshold: f32,
    pub enable_validation: bool,
    pub validation_threshold: f32,
    pub performance_thresholds: PerformanceThresholds,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.8,
            max_candidates: 5,
            use_cache: true,
            cache_size: 1000,
            enable_neural: true,
            neural_threshold: 0.7,
            enable_validation: true,
            validation_threshold: 0.9,
            performance_thresholds: PerformanceThresholds::default(),
        }
    }
}

impl AnalyzerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_min_confidence(mut self, confidence: f32) -> Self {
        self.min_confidence = confidence;
        self
    }

    pub fn with_max_candidates(mut self, candidates: usize) -> Self {
        self.max_candidates = candidates;
        self
    }

    pub fn with_cache_settings(mut self, use_cache: bool, cache_size: usize) -> Self {
        self.use_cache = use_cache;
        self.cache_size = cache_size;
        self
    }

    pub fn with_neural_settings(mut self, enable: bool, threshold: f32) -> Self {
        self.enable_neural = enable;
        self.neural_threshold = threshold;
        self
    }

    pub fn with_validation_settings(mut self, enable: bool, threshold: f32) -> Self {
        self.enable_validation = enable;
        self.validation_threshold = threshold;
        self
    }

    pub fn with_performance_thresholds(mut self, thresholds: PerformanceThresholds) -> Self {
        self.performance_thresholds = thresholds;
        self
    }

    pub fn meets_confidence_threshold(&self, confidence: f32) -> bool {
        confidence >= self.min_confidence
    }

    pub fn meets_neural_threshold(&self, confidence: f32) -> bool {
        !self.enable_neural || confidence >= self.neural_threshold
    }

    pub fn meets_validation_threshold(&self, confidence: f32) -> bool {
        !self.enable_validation || confidence >= self.validation_threshold
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HebrewMorphology {
    pub root: String,
    pub pattern: String,
    pub gender: Option<Gender>,
    pub number: Option<Number>,
    pub person: Option<Person>,
    pub tense: Option<Tense>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RussianMorphology {
    pub stem: String,
    pub suffix: String,
    pub gender: Option<Gender>,
    pub number: Option<Number>,
    pub person: Option<Person>,
    pub tense: Option<Tense>,
    pub aspect: Option<Aspect>,
    pub confidence: f64,
}

impl HebrewMorphology {
    pub fn new(root: String, pattern: String) -> Self {
        Self {
            root,
            pattern,
            gender: None,
            number: None,
            person: None,
            tense: None,
            confidence: 1.0,
        }
    }
}

impl RussianMorphology {
    pub fn new(stem: String, suffix: String) -> Self {
        Self {
            stem,
            suffix,
            gender: None,
            number: None,
            person: None,
            tense: None,
            aspect: None,
            confidence: 1.0,
        }
    }
} 