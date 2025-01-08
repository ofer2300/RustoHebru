use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::morphology::{HebrewMorphology, RussianMorphology};
use crate::quality_control::ValidationReport;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::SystemTime;
use crate::neural::model::EnhancedTransformer;
use crate::neural::training::{Trainer, TrainingConfig, TrainingMetrics};
use crate::evaluation::{Evaluator, EvaluationMetrics};
use crate::technical_dictionary::TechnicalDictionary;

#[derive(Debug, Clone)]
pub enum LearningEventType {
    Translation {
        source_language: String,
        target_language: String,
    },
    Correction {
        original: String,
        corrected: String,
        error_type: String,
    },
    ValidationFailure {
        reason: String,
        severity: ValidationSeverity,
    },
    Success {
        improvement_type: String,
        confidence: f32,
    },
    UserFeedback {
        rating: i32,
        comments: String,
    },
}

#[derive(Debug, Clone)]
pub enum ValidationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl LearningEventType {
    pub fn is_translation(&self) -> bool {
        matches!(self, LearningEventType::Translation { .. })
    }

    pub fn is_correction(&self) -> bool {
        matches!(self, LearningEventType::Correction { .. })
    }

    pub fn is_validation_failure(&self) -> bool {
        matches!(self, LearningEventType::ValidationFailure { .. })
    }

    pub fn is_success(&self) -> bool {
        matches!(self, LearningEventType::Success { .. })
    }

    pub fn is_user_feedback(&self) -> bool {
        matches!(self, LearningEventType::UserFeedback { .. })
    }

    pub fn get_severity(&self) -> Option<ValidationSeverity> {
        match self {
            LearningEventType::ValidationFailure { severity, .. } => Some(severity.clone()),
            _ => None,
        }
    }

    pub fn get_confidence(&self) -> Option<f32> {
        match self {
            LearningEventType::Success { confidence, .. } => Some(*confidence),
            _ => None,
        }
    }

    pub fn get_languages(&self) -> Option<(String, String)> {
        match self {
            LearningEventType::Translation { source_language, target_language } => {
                Some((source_language.clone(), target_language.clone()))
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
    pub confidence_score: f64,
}

#[derive(Debug, Clone)]
pub struct LearningEvent {
    pub event_type: LearningEventType,
    pub source_text: String,
    pub target_text: String,
    pub timestamp: SystemTime,
    pub metrics: EventMetrics,
    pub domain: String,
    pub context: String,
}

impl LearningEvent {
    pub fn new(
        event_type: LearningEventType,
        source_text: String,
        target_text: String,
        metrics: EventMetrics,
        domain: String,
        context: String,
    ) -> Self {
        Self {
            event_type,
            source_text,
            target_text,
            timestamp: SystemTime::now(),
            metrics,
            domain,
            context,
        }
    }

    pub fn with_event_type(mut self, event_type: LearningEventType) -> Self {
        self.event_type = event_type;
        self
    }

    pub fn with_source_text(mut self, source_text: String) -> Self {
        self.source_text = source_text;
        self
    }

    pub fn with_target_text(mut self, target_text: String) -> Self {
        self.target_text = target_text;
        self
    }

    pub fn with_metrics(mut self, metrics: EventMetrics) -> Self {
        self.metrics = metrics;
        self
    }

    pub fn with_domain(mut self, domain: String) -> Self {
        self.domain = domain;
        self
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = context;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    pub rating: u8,
    pub comments: Option<String>,
    pub corrections: Option<Vec<Correction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correction {
    pub original_text: String,
    pub corrected_text: String,
    pub correction_type: CorrectionType,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CorrectionType {
    Grammar,
    Style,
    Terminology,
    Cultural,
    Other,
}

pub struct DomainAdapter {
    domains: HashMap<String, f32>,
}

pub struct ContinuousLearner {
    model: HashMap<String, f32>,
}

pub struct PerformanceMonitor {
    metrics: HashMap<String, f32>,
}

pub struct OptimizationEngine {
    parameters: HashMap<String, f32>,
}

impl DomainAdapter {
    pub fn new() -> Self {
        Self {
            domains: HashMap::new()
        }
    }

    pub fn adapt_from_feedback(&self, feedback: &LearningEvent, metrics: &EventMetrics) -> Result<EventMetrics, String> {
        Ok(metrics.clone())
    }
}

impl ContinuousLearner {
    pub fn new() -> Self {
        Self {
            model: HashMap::new()
        }
    }

    pub fn learn_from_translation(&self, event: &LearningEvent, metrics: &EventMetrics) -> Result<(), String> {
        Ok(())
    }

    pub fn learn_from_feedback(&self, feedback: &LearningEvent, metrics: &EventMetrics) -> Result<(), String> {
        Ok(())
    }
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new()
        }
    }

    pub fn start_operation(&self, name: &str) -> Result<(), String> {
        Ok(())
    }

    pub fn get_metrics(&self) -> Result<EventMetrics, String> {
        Ok(EventMetrics {
            success_rate: 0.0,
            average_confidence: 0.0,
            positive_feedback_rate: 0.0,
            domain_coverage: HashMap::new(),
            learning_progress: 0.0,
        })
    }
}

impl OptimizationEngine {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new()
        }
    }

    pub fn optimize(&self, metrics: &EventMetrics) -> Result<(), String> {
        Ok(())
    }

    pub fn get_metrics(&self) -> Result<EventMetrics, String> {
        Ok(EventMetrics {
            success_rate: 0.0,
            average_confidence: 0.0,
            positive_feedback_rate: 0.0,
            domain_coverage: HashMap::new(),
            learning_progress: 0.0,
        })
    }
}

pub struct AdvancedLearningManager {
    events: Arc<Mutex<Vec<LearningEvent>>>,
    hebrew_patterns: Arc<Mutex<HebrewPatternLearner>>,
    russian_patterns: Arc<Mutex<RussianPatternLearner>>,
    feedback_analyzer: Arc<Mutex<FeedbackAnalyzer>>,
    domain_adapter: Arc<Mutex<DomainAdapter>>,
    continuous_learner: Arc<Mutex<ContinuousLearner>>,
    performance_monitor: Arc<Mutex<PerformanceMonitor>>,
    optimization_engine: Arc<Mutex<OptimizationEngine>>,
}

#[derive(Debug, Default)]
struct HebrewPatternLearner {
    morphology_patterns: Vec<(HebrewMorphology, f64)>,
    common_mistakes: Vec<(String, String, f64)>,
    style_patterns: Vec<(String, f64)>,
}

#[derive(Debug, Default)]
struct RussianPatternLearner {
    morphology_patterns: Vec<(RussianMorphology, f64)>,
    case_patterns: Vec<(String, String, f64)>,
    style_patterns: Vec<(String, f64)>,
}

#[derive(Debug, Default)]
struct FeedbackAnalyzer {
    total_feedback: usize,
    positive_feedback: usize,
    negative_feedback: usize,
    common_issues: Vec<(String, usize)>,
}

impl AdvancedLearningManager {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            hebrew_patterns: Arc::new(Mutex::new(HebrewPatternLearner::default())),
            russian_patterns: Arc::new(Mutex::new(RussianPatternLearner::default())),
            feedback_analyzer: Arc::new(Mutex::new(FeedbackAnalyzer::default())),
            domain_adapter: Arc::new(Mutex::new(DomainAdapter::new())),
            continuous_learner: Arc::new(Mutex::new(ContinuousLearner::new())),
            performance_monitor: Arc::new(Mutex::new(PerformanceMonitor::new())),
            optimization_engine: Arc::new(Mutex::new(OptimizationEngine::new())),
        }
    }

    pub async fn record_event(&self, event: LearningEvent) -> Result<EventMetrics> {
        // מדידת ביצועים
        let _perf = self.performance_monitor.lock().await.start_operation("record_event");
        
        // תיעוד האירוע
        let mut events = self.events.lock().await;
        events.push(event.clone());
        
        // ניתוח האירוע
        let metrics = match event.event_type {
            LearningEventType::Translation => {
                self.analyze_translation(&event).await?
            }
            LearningEventType::Correction => {
                self.analyze_correction(&event).await?
            }
            LearningEventType::Feedback => {
                self.analyze_feedback(&event).await?
            }
            LearningEventType::ValidationFailure => {
                self.analyze_failure(&event).await?
            }
            LearningEventType::Success => {
                self.analyze_success(&event).await?
            }
        };

        // אופטימיזציה מתמשכת
        self.optimization_engine.lock().await.optimize(&metrics).await?;
        
        Ok(metrics)
    }

    async fn analyze_translation(&self, event: &LearningEvent) -> Result<EventMetrics> {
        let mut metrics = EventMetrics::new();
        
        if let Some(report) = &event.validation_report {
            // ניתוח דפוסים מורפולוגיים
            let mut hebrew_patterns = self.hebrew_patterns.lock().await;
            let hebrew_metrics = hebrew_patterns.analyze_morphology_enhanced(
                &event.source_text,
                &event.target_text,
                report
            ).await?;
            metrics.merge(hebrew_metrics);
            
            let mut russian_patterns = self.russian_patterns.lock().await;
            let russian_metrics = russian_patterns.analyze_morphology_enhanced(
                &event.source_text,
                &event.target_text,
                report
            ).await?;
            metrics.merge(russian_metrics);
            
            // ניתוח דפוסי סגנון
            let style_metrics = self.analyze_style_patterns(
                &event.source_text,
                &event.target_text,
                report
            ).await?;
            metrics.merge(style_metrics);
            
            // למידה מתמשכת
            let mut learner = self.continuous_learner.lock().await;
            learner.learn_from_translation(event, &metrics).await?;
        }
        
        Ok(metrics)
    }

    async fn analyze_correction(&self, event: &LearningEvent) -> Result<EventMetrics> {
        let mut metrics = EventMetrics::new();
        
        if let Some(feedback) = &event.user_feedback {
            if let Some(corrections) = &feedback.corrections {
                for correction in corrections {
                    match correction.correction_type {
                        CorrectionType::Grammar => {
                            let grammar_metrics = self.analyze_grammar_correction(
                                correction,
                                &event.source_text,
                                &event.target_text
                            ).await?;
                            metrics.merge(grammar_metrics);
                        }
                        CorrectionType::Style => {
                            let style_metrics = self.analyze_style_correction(
                                correction,
                                &event.source_text,
                                &event.target_text
                            ).await?;
                            metrics.merge(style_metrics);
                        }
                        CorrectionType::Terminology => {
                            let term_metrics = self.analyze_terminology_correction(
                                correction,
                                &event.source_text,
                                &event.target_text
                            ).await?;
                            metrics.merge(term_metrics);
                        }
                        CorrectionType::Cultural => {
                            let cultural_metrics = self.analyze_cultural_correction(
                                correction,
                                &event.source_text,
                                &event.target_text
                            ).await?;
                            metrics.merge(cultural_metrics);
                        }
                        _ => {}
                    }
                }
            }
        }
        
        Ok(metrics)
    }

    async fn analyze_feedback(&self, event: &LearningEvent) -> Result<EventMetrics> {
        let mut metrics = EventMetrics::new();
        
        if let Some(feedback) = &event.user_feedback {
            let mut analyzer = self.feedback_analyzer.lock().await;
            
            // עיתוח מתקדם של משוב
            let feedback_metrics = analyzer.analyze_feedback_enhanced(
                feedback,
                &event.source_text,
                &event.target_text
            ).await?;
            metrics.merge(feedback_metrics);
            
            // התאמת דומיין
            let mut domain_adapter = self.domain_adapter.lock().await;
            let domain_metrics = domain_adapter.adapt_from_feedback(
                feedback,
                &event.source_text,
                &event.target_text
            ).await?;
            metrics.merge(domain_metrics);
            
            // למידה מתמשכת
            let mut learner = self.continuous_learner.lock().await;
            learner.learn_from_feedback(feedback, &metrics).await?;
        }
        
        Ok(metrics)
    }

    pub async fn get_learning_statistics(&self) -> Result<EnhancedLearningStatistics> {
        let events = self.events.lock().await;
        let analyzer = self.feedback_analyzer.lock().await;
        let monitor = self.performance_monitor.lock().await;
        let optimization = self.optimization_engine.lock().await;
        
        Ok(EnhancedLearningStatistics {
            total_events: events.len(),
            success_rate: self.calculate_success_rate(&events),
            average_confidence: self.calculate_average_confidence(&events),
            positive_feedback_rate: self.calculate_feedback_rate(&analyzer),
            performance_metrics: monitor.get_metrics().await?,
            optimization_metrics: optimization.get_metrics().await?,
            domain_coverage: self.calculate_domain_coverage().await?,
            learning_progress: self.calculate_learning_progress().await?,
        })
    }

    pub fn analyze_failure(&self, event: &LearningEvent) -> Result<EventMetrics, String> {
        Ok(EventMetrics {
            success_rate: 0.0,
            average_confidence: 0.0,
            positive_feedback_rate: 0.0,
            domain_coverage: HashMap::new(),
            learning_progress: 0.0,
        })
    }

    pub fn analyze_success(&self, event: &LearningEvent) -> Result<EventMetrics, String> {
        Ok(EventMetrics {
            success_rate: 1.0,
            average_confidence: 1.0,
            positive_feedback_rate: 1.0,
            domain_coverage: HashMap::new(),
            learning_progress: 1.0,
        })
    }

    pub fn analyze_style_patterns(&self, event: &LearningEvent) -> Result<EventMetrics, String> {
        Ok(EventMetrics {
            success_rate: 0.5,
            average_confidence: 0.5,
            positive_feedback_rate: 0.5,
            domain_coverage: HashMap::new(),
            learning_progress: 0.5,
        })
    }

    pub fn analyze_grammar_correction(&self, event: &LearningEvent) -> Result<EventMetrics, String> {
        Ok(EventMetrics {
            success_rate: 0.5,
            average_confidence: 0.5,
            positive_feedback_rate: 0.5,
            domain_coverage: HashMap::new(),
            learning_progress: 0.5,
        })
    }

    pub fn analyze_style_correction(&self, event: &LearningEvent) -> Result<EventMetrics, String> {
        Ok(EventMetrics {
            success_rate: 0.5,
            average_confidence: 0.5,
            positive_feedback_rate: 0.5,
            domain_coverage: HashMap::new(),
            learning_progress: 0.5,
        })
    }

    pub fn analyze_terminology_correction(&self, event: &LearningEvent) -> Result<EventMetrics, String> {
        Ok(EventMetrics {
            success_rate: 0.5,
            average_confidence: 0.5,
            positive_feedback_rate: 0.5,
            domain_coverage: HashMap::new(),
            learning_progress: 0.5,
        })
    }

    pub fn analyze_cultural_correction(&self, event: &LearningEvent) -> Result<EventMetrics, String> {
        Ok(EventMetrics {
            success_rate: 0.5,
            average_confidence: 0.5,
            positive_feedback_rate: 0.5,
            domain_coverage: HashMap::new(),
            learning_progress: 0.5,
        })
    }

    pub fn calculate_success_rate(&self, events: &[LearningEvent]) -> f64 {
        0.5
    }

    pub fn calculate_average_confidence(&self, events: &[LearningEvent]) -> f64 {
        0.5
    }

    pub fn calculate_feedback_rate(&self, analyzer: &str) -> f64 {
        0.5
    }

    pub fn calculate_domain_coverage(&self) -> Result<HashMap<String, f64>, String> {
        Ok(HashMap::new())
    }

    pub fn calculate_learning_progress(&self) -> Result<f64, String> {
        Ok(0.5)
    }
}

impl HebrewPatternLearner {
    fn analyze_morphology(&mut self, _source: &str, _target: &str) -> Result<()> {
        // TODO: יישום ניתוח מורפולוגי
        Ok(())
    }

    fn analyze_style(&mut self, _text: &str) -> Result<()> {
        // TODO: יישום ניתוח סגנון
        Ok(())
    }

    fn update_common_mistakes(&mut self, original: &str, corrected: &str) {
        if let Some(idx) = self.common_mistakes.iter()
            .position(|(o, c, _)| o == original && c == corrected) {
            self.common_mistakes[idx].2 += 1.0;
        } else {
            self.common_mistakes.push((
                original.to_string(),
                corrected.to_string(),
                1.0,
            ));
        }
    }

    fn update_style_patterns(&mut self, text: &str) {
        if let Some(idx) = self.style_patterns.iter()
            .position(|(p, _)| p == text) {
            self.style_patterns[idx].1 += 1.0;
        } else {
            self.style_patterns.push((text.to_string(), 1.0));
        }
    }

    fn analyze_failures(&mut self, _report: &ValidationReport) {
        // TODO: יישום ניתוח כשלים
    }

    fn analyze_successes(&mut self, _report: &ValidationReport) {
        // TODO: יישום ניתוח הצלחות
    }
}

impl RussianPatternLearner {
    fn analyze_morphology(&mut self, _source: &str, _target: &str) -> Result<()> {
        // TODO: יישום ניתוח מורפולוגי
        Ok(())
    }

    fn analyze_style(&mut self, _text: &str) -> Result<()> {
        // TODO: יישום ניתוח סגנון
        Ok(())
    }

    fn update_common_mistakes(&mut self, original: &str, corrected: &str) {
        if let Some(idx) = self.case_patterns.iter()
            .position(|(o, c, _)| o == original && c == corrected) {
            self.case_patterns[idx].2 += 1.0;
        } else {
            self.case_patterns.push((
                original.to_string(),
                corrected.to_string(),
                1.0,
            ));
        }
    }

    fn update_style_patterns(&mut self, text: &str) {
        if let Some(idx) = self.style_patterns.iter()
            .position(|(p, _)| p == text) {
            self.style_patterns[idx].1 += 1.0;
        } else {
            self.style_patterns.push((text.to_string(), 1.0));
        }
    }

    fn analyze_failures(&mut self, _report: &ValidationReport) {
        // TODO: יישום ניתוח כשלים
    }

    fn analyze_successes(&mut self, _report: &ValidationReport) {
        // TODO: יישום ניתוח הצלחות
    }
}

impl FeedbackAnalyzer {
    fn analyze_comments(&mut self, comments: &str) {
        // ניתוח פשוט של מילות מפתח בהערות
        let keywords = ["grammar", "style", "terminology", "cultural"];
        
        for keyword in keywords.iter() {
            if comments.to_lowercase().contains(keyword) {
                if let Some(idx) = self.common_issues.iter()
                    .position(|(issue, _)| issue == keyword) {
                    self.common_issues[idx].1 += 1;
                } else {
                    self.common_issues.push((keyword.to_string(), 1));
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventMetrics {
    pub processing_time_ms: u64,
    pub memory_usage_mb: u64,
    pub confidence_score: f32,
    pub accuracy_score: f32,
    pub error_count: u32,
    pub warning_count: u32,
    pub suggestion_count: u32,
    pub validation_status: ValidationStatus,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone)]
pub enum ValidationStatus {
    Success,
    Warning,
    Error,
    NotValidated,
}

impl Default for EventMetrics {
    fn default() -> Self {
        Self {
            processing_time_ms: 0,
            memory_usage_mb: 0,
            confidence_score: 0.0,
            accuracy_score: 0.0,
            error_count: 0,
            warning_count: 0,
            suggestion_count: 0,
            validation_status: ValidationStatus::NotValidated,
            performance_metrics: PerformanceMetrics::default(),
        }
    }
}

impl EventMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_processing_time(mut self, time_ms: u64) -> Self {
        self.processing_time_ms = time_ms;
        self
    }

    pub fn with_memory_usage(mut self, memory_mb: u64) -> Self {
        self.memory_usage_mb = memory_mb;
        self
    }

    pub fn with_confidence_score(mut self, confidence: f32) -> Self {
        self.confidence_score = confidence;
        self
    }

    pub fn with_accuracy_score(mut self, accuracy: f32) -> Self {
        self.accuracy_score = accuracy;
        self
    }

    pub fn with_error_count(mut self, errors: u32) -> Self {
        self.error_count = errors;
        self
    }

    pub fn with_warning_count(mut self, warnings: u32) -> Self {
        self.warning_count = warnings;
        self
    }

    pub fn with_suggestion_count(mut self, suggestions: u32) -> Self {
        self.suggestion_count = suggestions;
        self
    }

    pub fn with_validation_status(mut self, status: ValidationStatus) -> Self {
        self.validation_status = status;
        self
    }

    pub fn with_performance_metrics(mut self, metrics: PerformanceMetrics) -> Self {
        self.performance_metrics = metrics;
        self
    }

    pub fn update_from_validation_report(&mut self, report: &ValidationReport) {
        self.error_count = report.errors.len() as u32;
        self.warning_count = report.warnings.len() as u32;
        self.suggestion_count = report.suggestions.len() as u32;
        self.confidence_score = report.confidence_score as f32;
        self.validation_status = if report.is_valid {
            if !report.warnings.is_empty() {
                ValidationStatus::Warning
            } else {
                ValidationStatus::Success
            }
        } else {
            ValidationStatus::Error
        };
    }
}

#[derive(Debug, Clone)]
pub struct EnhancedLearningStatistics {
    pub total_events: usize,
    pub success_rate: f64,
    pub average_confidence: f64,
    pub positive_feedback_rate: f64,
    pub performance_metrics: PerformanceMetrics,
    pub optimization_metrics: OptimizationMetrics,
    pub domain_coverage: DomainCoverage,
    pub learning_progress: LearningProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub average_translation_time_ms: u64,
    pub peak_memory_usage_mb: u64,
    pub cache_hit_rate: f64,
    pub throughput_per_second: f64,
    pub success_rate: f64,
    pub average_confidence: f64,
    pub positive_feedback_rate: f64,
    pub domain_coverage: HashMap<String, f64>,
    pub learning_progress: f64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            average_translation_time_ms: 0,
            peak_memory_usage_mb: 0,
            cache_hit_rate: 0.0,
            throughput_per_second: 0.0,
            success_rate: 0.0,
            average_confidence: 0.0,
            positive_feedback_rate: 0.0,
            domain_coverage: HashMap::new(),
            learning_progress: 0.0,
        }
    }
}

impl PerformanceMetrics {
    pub fn merge(&mut self, other: &PerformanceMetrics) {
        self.average_translation_time_ms = (self.average_translation_time_ms + other.average_translation_time_ms) / 2;
        self.peak_memory_usage_mb = self.peak_memory_usage_mb.max(other.peak_memory_usage_mb);
        self.cache_hit_rate = (self.cache_hit_rate + other.cache_hit_rate) / 2.0;
        self.throughput_per_second = (self.throughput_per_second + other.throughput_per_second) / 2.0;
        self.success_rate = (self.success_rate + other.success_rate) / 2.0;
        self.average_confidence = (self.average_confidence + other.average_confidence) / 2.0;
        self.positive_feedback_rate = (self.positive_feedback_rate + other.positive_feedback_rate) / 2.0;
        
        for (domain, coverage) in &other.domain_coverage {
            let entry = self.domain_coverage.entry(domain.clone()).or_insert(0.0);
            *entry = (*entry + coverage) / 2.0;
        }
        
        self.learning_progress = (self.learning_progress + other.learning_progress) / 2.0;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationMetrics {
    pub convergence_rate: f64,
    pub stability_score: f64,
    pub resource_usage: f64,
}

impl OptimizationMetrics {
    pub fn new() -> Self {
        Self {
            convergence_rate: 0.0,
            stability_score: 0.0,
            resource_usage: 0.0,
        }
    }

    pub fn merge(&mut self, other: &OptimizationMetrics) {
        self.convergence_rate = (self.convergence_rate + other.convergence_rate) / 2.0;
        self.stability_score = (self.stability_score + other.stability_score) / 2.0;
        self.resource_usage = (self.resource_usage + other.resource_usage) / 2.0;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainCoverage {
    pub domain: String,
    pub coverage_percent: f32,
    pub total_samples: u32,
    pub successful_samples: u32,
    pub error_rate: f32,
    pub confidence_score: f32,
    pub last_update: SystemTime,
}

impl DomainCoverage {
    pub fn new(domain: String) -> Self {
        Self {
            domain,
            coverage_percent: 0.0,
            total_samples: 0,
            successful_samples: 0,
            error_rate: 0.0,
            confidence_score: 0.0,
            last_update: SystemTime::now(),
        }
    }

    pub fn update_stats(&mut self, success: bool, confidence: f32) {
        self.total_samples += 1;
        if success {
            self.successful_samples += 1;
        }
        
        self.coverage_percent = (self.successful_samples as f32 / self.total_samples as f32) * 100.0;
        self.error_rate = 1.0 - (self.successful_samples as f32 / self.total_samples as f32);
        self.confidence_score = (self.confidence_score * (self.total_samples - 1) as f32 + confidence) 
            / self.total_samples as f32;
        self.last_update = SystemTime::now();
    }

    pub fn meets_threshold(&self, min_coverage: f32, max_error_rate: f32) -> bool {
        self.coverage_percent >= min_coverage && self.error_rate <= max_error_rate
    }

    pub fn reset_stats(&mut self) {
        self.coverage_percent = 0.0;
        self.total_samples = 0;
        self.successful_samples = 0;
        self.error_rate = 0.0;
        self.confidence_score = 0.0;
        self.last_update = SystemTime::now();
    }

    pub fn merge(&mut self, other: &DomainCoverage) {
        if self.domain != other.domain {
            return;
        }

        let total = self.total_samples + other.total_samples;
        if total == 0 {
            return;
        }

        self.successful_samples += other.successful_samples;
        self.total_samples = total;
        self.coverage_percent = (self.successful_samples as f32 / total as f32) * 100.0;
        self.error_rate = 1.0 - (self.successful_samples as f32 / total as f32);
        self.confidence_score = (self.confidence_score * self.total_samples as f32 
            + other.confidence_score * other.total_samples as f32) / total as f32;
        self.last_update = SystemTime::now();
    }
}

#[derive(Debug, Clone)]
pub struct LearningProgress {
    pub total_events: u32,
    pub successful_events: u32,
    pub error_events: u32,
    pub success_rate: f32,
    pub average_confidence: f32,
    pub improvement_rate: f32,
    pub domain_coverage: HashMap<String, DomainCoverage>,
    pub last_update: SystemTime,
}

impl Default for LearningProgress {
    fn default() -> Self {
        Self {
            total_events: 0,
            successful_events: 0,
            error_events: 0,
            success_rate: 0.0,
            average_confidence: 0.0,
            improvement_rate: 0.0,
            domain_coverage: HashMap::new(),
            last_update: SystemTime::now(),
        }
    }
}

impl LearningProgress {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_from_event(&mut self, event: &LearningEvent) {
        self.total_events += 1;
        
        match event.event_type {
            LearningEventType::Success { confidence, .. } => {
                self.successful_events += 1;
                self.update_confidence(confidence);
            },
            LearningEventType::ValidationFailure { .. } => {
                self.error_events += 1;
            },
            _ => {}
        }

        self.update_rates();
        self.update_domain_coverage(&event.domain, event.metrics.confidence_score);
        self.last_update = SystemTime::now();
    }

    fn update_confidence(&mut self, new_confidence: f32) {
        self.average_confidence = (self.average_confidence * (self.total_events - 1) as f32 
            + new_confidence) / self.total_events as f32;
    }

    fn update_rates(&mut self) {
        if self.total_events > 0 {
            self.success_rate = self.successful_events as f32 / self.total_events as f32;
            self.improvement_rate = (self.successful_events as f32 - self.error_events as f32) 
                / self.total_events as f32;
        }
    }

    fn update_domain_coverage(&mut self, domain: &str, confidence: f32) {
        let coverage = self.domain_coverage
            .entry(domain.to_string())
            .or_insert_with(|| DomainCoverage::new(domain.to_string()));
        
        coverage.update_stats(confidence >= 0.8, confidence);
    }

    pub fn get_domain_coverage(&self, domain: &str) -> Option<&DomainCoverage> {
        self.domain_coverage.get(domain)
    }

    pub fn merge(&mut self, other: &LearningProgress) {
        self.total_events += other.total_events;
        self.successful_events += other.successful_events;
        self.error_events += other.error_events;
        
        for (domain, coverage) in &other.domain_coverage {
            let entry = self.domain_coverage
                .entry(domain.clone())
                .or_insert_with(|| DomainCoverage::new(domain.clone()));
            entry.merge(coverage);
        }

        self.update_rates();
        self.last_update = SystemTime::now();
    }

    pub fn reset_stats(&mut self) {
        self.total_events = 0;
        self.successful_events = 0;
        self.error_events = 0;
        self.success_rate = 0.0;
        self.average_confidence = 0.0;
        self.improvement_rate = 0.0;
        self.domain_coverage.clear();
        self.last_update = SystemTime::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    pub batch_size: usize,
    pub learning_rate: f64,
    pub num_epochs: usize,
    pub validation_split: f64,
    pub min_accuracy: f64,
    pub max_iterations: usize,
    pub improvement_threshold: f64,
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            batch_size: 32,
            learning_rate: 0.001,
            num_epochs: 10,
            validation_split: 0.2,
            min_accuracy: 0.95,
            max_iterations: 5,
            improvement_threshold: 0.01,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningMetrics {
    pub iteration: usize,
    pub training_metrics: Vec<TrainingMetrics>,
    pub evaluation_metrics: EvaluationMetrics,
    pub improvements: Vec<Improvement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Improvement {
    pub iteration: usize,
    pub metric_name: String,
    pub previous_value: f64,
    pub new_value: f64,
    pub relative_improvement: f64,
}

pub struct LearningManager {
    model: Arc<Mutex<EnhancedTransformer>>,
    trainer: Arc<Mutex<Trainer>>,
    evaluator: Arc<Mutex<Evaluator>>,
    technical_dictionary: Arc<Mutex<TechnicalDictionary>>,
    config: LearningConfig,
    metrics: Vec<LearningMetrics>,
}

impl LearningManager {
    pub fn new(
        model: EnhancedTransformer,
        technical_dictionary: TechnicalDictionary,
        config: Option<LearningConfig>,
    ) -> Self {
        let config = config.unwrap_or_default();
        
        let training_config = TrainingConfig {
            batch_size: config.batch_size,
            learning_rate: config.learning_rate,
            num_epochs: config.num_epochs,
            validation_split: config.validation_split,
            ..Default::default()
        };
        
        let trainer = Trainer::new(model.clone(), training_config);
        
        let evaluator = Evaluator::new(
            HashMap::new(), // יש להוסיף תרגומי ייחוס
            technical_dictionary.get_terms(),
            Default::default(), // יש להוסיף הגדרות סגנון
        );
        
        Self {
            model: Arc::new(Mutex::new(model)),
            trainer: Arc::new(Mutex::new(trainer)),
            evaluator: Arc::new(Mutex::new(evaluator)),
            technical_dictionary: Arc::new(Mutex::new(technical_dictionary)),
            config,
            metrics: Vec::new(),
        }
    }
    
    pub async fn learn(&mut self, training_data: Vec<(String, String)>) -> Result<(), LearningError> {
        let mut current_iteration = 0;
        let mut best_accuracy = 0.0;
        
        while current_iteration < self.config.max_iterations {
            println!("התחלת איטרציה {}", current_iteration + 1);
            
            // אימון המודל
            let training_metrics = self.trainer.lock().unwrap()
                .train(training_data.clone())?;
                
            // הערכת ביצועים
            let evaluation_metrics = self.evaluate_model(&training_data)?;
            
            // חישוב שיפורים
            let improvements = self.calculate_improvements(
                current_iteration,
                &training_metrics,
                &evaluation_metrics,
                best_accuracy,
            );
            
            // עדכון המטריקות
            self.metrics.push(LearningMetrics {
                iteration: current_iteration,
                training_metrics: training_metrics.clone(),
                evaluation_metrics: evaluation_metrics.clone(),
                improvements: improvements.clone(),
            });
            
            // בדיקת תנאי עצירה
            if evaluation_metrics.technical_accuracy >= self.config.min_accuracy {
                println!("הושגה רמת דיוק מספקת");
                break;
            }
            
            let current_accuracy = evaluation_metrics.technical_accuracy;
            let improvement = current_accuracy - best_accuracy;
            
            if improvement < self.config.improvement_threshold {
                println!("לא נמצא שיפור משמעותי");
                break;
            }
            
            best_accuracy = best_accuracy.max(current_accuracy);
            current_iteration += 1;
            
            // עדכון המילון הטכני
            self.update_technical_dictionary(&training_data)?;
        }
        
        Ok(())
    }
    
    fn evaluate_model(&self, test_data: &[(String, String)]) -> Result<EvaluationMetrics, LearningError> {
        let evaluator = self.evaluator.lock().unwrap();
        let model = self.model.lock().unwrap();
        
        let mut total_metrics = EvaluationMetrics {
            bleu_score: 0.0,
            meteor_score: 0.0,
            ter_score: 0.0,
            chrf_score: 0.0,
            technical_accuracy: 0.0,
            fluency_score: 0.0,
            adequacy_score: 0.0,
            error_analysis: Default::default(),
        };
        
        for (source, reference) in test_data {
            let translation = model.translate_text(source)
                .map_err(|e| LearningError::TranslationError(e.to_string()))?;
                
            let metrics = evaluator.evaluate(source, &translation.text);
            
            total_metrics.bleu_score += metrics.bleu_score;
            total_metrics.meteor_score += metrics.meteor_score;
            total_metrics.ter_score += metrics.ter_score;
            total_metrics.chrf_score += metrics.chrf_score;
            total_metrics.technical_accuracy += metrics.technical_accuracy;
            total_metrics.fluency_score += metrics.fluency_score;
            total_metrics.adequacy_score += metrics.adequacy_score;
        }
        
        let num_samples = test_data.len() as f64;
        
        total_metrics.bleu_score /= num_samples;
        total_metrics.meteor_score /= num_samples;
        total_metrics.ter_score /= num_samples;
        total_metrics.chrf_score /= num_samples;
        total_metrics.technical_accuracy /= num_samples;
        total_metrics.fluency_score /= num_samples;
        total_metrics.adequacy_score /= num_samples;
        
        Ok(total_metrics)
    }
    
    fn calculate_improvements(
        &self,
        iteration: usize,
        training_metrics: &[TrainingMetrics],
        evaluation_metrics: &EvaluationMetrics,
        previous_accuracy: f64,
    ) -> Vec<Improvement> {
        let mut improvements = Vec::new();
        
        // שיפור בדיוק הטכני
        let accuracy_improvement = evaluation_metrics.technical_accuracy - previous_accuracy;
        if accuracy_improvement > 0.0 {
            improvements.push(Improvement {
                iteration,
                metric_name: "technical_accuracy".to_string(),
                previous_value: previous_accuracy,
                new_value: evaluation_metrics.technical_accuracy,
                relative_improvement: accuracy_improvement / previous_accuracy,
            });
        }
        
        // שיפור ב-BLEU
        if let Some(previous_metrics) = self.metrics.last() {
            let bleu_improvement = evaluation_metrics.bleu_score - previous_metrics.evaluation_metrics.bleu_score;
            if bleu_improvement > 0.0 {
                improvements.push(Improvement {
                    iteration,
                    metric_name: "bleu_score".to_string(),
                    previous_value: previous_metrics.evaluation_metrics.bleu_score,
                    new_value: evaluation_metrics.bleu_score,
                    relative_improvement: bleu_improvement / previous_metrics.evaluation_metrics.bleu_score,
                });
            }
        }
        
        improvements
    }
    
    fn update_technical_dictionary(&mut self, training_data: &[(String, String)]) -> Result<(), LearningError> {
        let mut dictionary = self.technical_dictionary.lock().unwrap();
        
        for (source, target) in training_data {
            // זיהוי מונחים טכניים חדשים
            let source_terms = dictionary.extract_terms(source);
            let target_terms = dictionary.extract_terms(target);
            
            for (source_term, target_term) in source_terms.iter().zip(target_terms.iter()) {
                dictionary.add_term(
                    source_term,
                    target_term,
                    "auto_detected", // תחום ברירת מחדל
                    "", // הקשר ריק
                );
            }
        }
        
        Ok(())
    }
    
    pub fn get_metrics(&self) -> &[LearningMetrics] {
        &self.metrics
    }
    
    pub fn save_state(&self, path: &str) -> Result<(), LearningError> {
        // שמירת המודל
        self.model.lock().unwrap()
            .save_model(path)
            .map_err(|e| LearningError::ModelError(e.to_string()))?;
            
        // שמירת המילון הטכני
        let dictionary_path = format!("{}_dictionary.json", path);
        self.technical_dictionary.lock().unwrap()
            .save(&dictionary_path)
            .map_err(|e| LearningError::DictionaryError(e.to_string()))?;
            
        // שמירת מטריקות
        let metrics_path = format!("{}_metrics.json", path);
        std::fs::write(
            &metrics_path,
            serde_json::to_string_pretty(&self.metrics)
                .map_err(|e| LearningError::SerializationError(e.to_string()))?
        ).map_err(|e| LearningError::IoError(e.to_string()))?;
        
        Ok(())
    }
    
    pub fn load_state(path: &str) -> Result<Self, LearningError> {
        // טעינת המודל
        let model = EnhancedTransformer::load_model(path)
            .map_err(|e| LearningError::ModelError(e.to_string()))?;
            
        // טעינת המילון הטכני
        let dictionary_path = format!("{}_dictionary.json", path);
        let technical_dictionary = TechnicalDictionary::load(&dictionary_path)
            .map_err(|e| LearningError::DictionaryError(e.to_string()))?;
            
        // טעינת מטריקות
        let metrics_path = format!("{}_metrics.json", path);
        let metrics: Vec<LearningMetrics> = serde_json::from_str(
            &std::fs::read_to_string(&metrics_path)
                .map_err(|e| LearningError::IoError(e.to_string()))?
        ).map_err(|e| LearningError::DeserializationError(e.to_string()))?;
        
        let mut manager = Self::new(model, technical_dictionary, None);
        manager.metrics = metrics;
        
        Ok(manager)
    }
}

#[derive(Debug)]
pub enum LearningError {
    TranslationError(String),
    ModelError(String),
    DictionaryError(String),
    SerializationError(String),
    DeserializationError(String),
    IoError(String),
}

impl std::fmt::Display for LearningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TranslationError(msg) => write!(f, "שגיאת תרגום: {}", msg),
            Self::ModelError(msg) => write!(f, "שגיאת מודל: {}", msg),
            Self::DictionaryError(msg) => write!(f, "שגיאת מילון: {}", msg),
            Self::SerializationError(msg) => write!(f, "שגיאת סריאליזציה: {}", msg),
            Self::DeserializationError(msg) => write!(f, "שגיאת דה-סריאליזציה: {}", msg),
            Self::IoError(msg) => write!(f, "שגיאת קלט/פלט: {}", msg),
        }
    }
}

impl std::error::Error for LearningError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_record_event() {
        let manager = AdvancedLearningManager::new();
        let event = LearningEvent {
            timestamp: Utc::now(),
            event_type: LearningEventType::Translation,
            source_text: "Hello".to_string(),
            target_text: "שלום".to_string(),
            validation_report: None,
            user_feedback: None,
            confidence_score: 0.9,
            metrics: EventMetrics::new(),
        };
        
        manager.record_event(event).await.unwrap();
        
        let stats = manager.get_learning_statistics().await.unwrap();
        assert_eq!(stats.total_events, 1);
    }

    #[tokio::test]
    async fn test_feedback_analysis() {
        let manager = AdvancedLearningManager::new();
        let event = LearningEvent {
            timestamp: Utc::now(),
            event_type: LearningEventType::Feedback,
            source_text: "Hello".to_string(),
            target_text: "שלום".to_string(),
            validation_report: None,
            user_feedback: Some(UserFeedback {
                rating: 5,
                comments: Some("Great translation!".to_string()),
                corrections: None,
            }),
            confidence_score: 0.9,
            metrics: EventMetrics::new(),
        };
        
        manager.record_event(event).await.unwrap();
        
        let stats = manager.get_learning_statistics().await.unwrap();
        assert!(stats.positive_feedback_rate > 0.0);
    }
} 