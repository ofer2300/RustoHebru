use std::time::Instant;
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use std::collections::HashMap;

const HISTORY_SIZE: usize = 1000; // שומר 1000 מדידות אחרונות

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub success_rate: f64,
    pub average_confidence: f64,
    pub positive_feedback_rate: f64,
    pub domain_coverage: HashMap<String, f64>,
    pub learning_progress: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Success,
    Warning,
    Error,
    NotValidated,
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    pub translation_time_ms: u64,
    pub memory_usage_mb: u64,
    pub cpu_usage_percent: f32,
    pub bleu_score: f32,
    pub rouge_score: f32,
    pub meteor_score: f32,
    pub model_confidence: f32,
    pub error_rate: f32,
    pub success_rate: f32,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_translation_time(&mut self, time_ms: u64) {
        self.translation_time_ms = time_ms;
    }

    pub fn update_memory_usage(&mut self, memory_mb: u64) {
        self.memory_usage_mb = memory_mb;
    }

    pub fn update_cpu_usage(&mut self, cpu_percent: f32) {
        self.cpu_usage_percent = cpu_percent;
    }

    pub fn update_bleu_score(&mut self, bleu: f32) {
        self.bleu_score = bleu;
    }

    pub fn update_rouge_score(&mut self, rouge: f32) {
        self.rouge_score = rouge;
    }

    pub fn update_meteor_score(&mut self, meteor: f32) {
        self.meteor_score = meteor;
    }

    pub fn update_model_confidence(&mut self, confidence: f32) {
        self.model_confidence = confidence;
    }

    pub fn update_error_rate(&mut self, error_rate: f32) {
        self.error_rate = error_rate;
    }

    pub fn update_success_rate(&mut self, success_rate: f32) {
        self.success_rate = success_rate;
    }

    pub fn meets_thresholds(&self, thresholds: &PerformanceThresholds) -> bool {
        self.translation_time_ms <= thresholds.max_translation_time_ms &&
        self.memory_usage_mb <= thresholds.max_memory_usage_mb &&
        self.cpu_usage_percent <= thresholds.max_cpu_usage_percent &&
        self.bleu_score >= thresholds.min_bleu_score &&
        self.rouge_score >= thresholds.min_rouge_score &&
        self.meteor_score >= thresholds.min_meteor_score &&
        self.model_confidence >= thresholds.min_model_confidence &&
        self.error_rate <= thresholds.max_error_rate &&
        self.success_rate >= thresholds.min_success_rate
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationMetrics {
    pub current_learning_rate: f64,
    pub parameter_updates: u64,
    pub gradient_norm: f64,
    pub optimization_steps: u64,
}

impl Default for EventMetrics {
    fn default() -> Self {
        Self {
            grammar_score: 0.0,
            style_score: 0.0,
            terminology_score: 0.0,
            cultural_score: 0.0,
            success_rate: 0.0,
            average_confidence: 0.0,
            positive_feedback_rate: 0.0,
            domain_coverage: HashMap::new(),
            learning_progress: 0.0,
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            translation_time_ms: 0,
            memory_usage_mb: 0,
            cpu_usage_percent: 0.0,
            bleu_score: 0.0,
            rouge_score: 0.0,
            meteor_score: 0.0,
            model_confidence: 0.0,
            error_rate: 0.0,
            success_rate: 0.0,
        }
    }
}

impl Default for OptimizationMetrics {
    fn default() -> Self {
        Self {
            current_learning_rate: 0.0,
            parameter_updates: 0,
            gradient_norm: 0.0,
            optimization_steps: 0,
        }
    }
}

#[derive(Debug)]
pub struct MetricsCollector {
    current_metrics: PerformanceMetrics,
    start_time: Option<Instant>,
    history: VecDeque<PerformanceMetrics>,
    best_metrics: PerformanceMetrics,
    worst_metrics: PerformanceMetrics,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            current_metrics: PerformanceMetrics::default(),
            start_time: None,
            history: VecDeque::with_capacity(HISTORY_SIZE),
            best_metrics: PerformanceMetrics::default(),
            worst_metrics: PerformanceMetrics::default(),
        }
    }

    pub fn record_metrics(&mut self) {
        // שמירת המדדים הנוכחיים בהיסטוריה
        self.history.push_back(self.current_metrics.clone());
        if self.history.len() > HISTORY_SIZE {
            self.history.pop_front();
        }

        // עדכון מדדי מקסימום ומינימום
        self.update_extremes();
    }

    fn update_extremes(&mut self) {
        // עדכון מדדים מיטביים
        if self.current_metrics.bleu_score > self.best_metrics.bleu_score {
            self.best_metrics.bleu_score = self.current_metrics.bleu_score;
        }
        if self.current_metrics.model_confidence > self.best_metrics.model_confidence {
            self.best_metrics.model_confidence = self.current_metrics.model_confidence;
        }
        if self.current_metrics.translation_time_ms < self.best_metrics.translation_time_ms || self.best_metrics.translation_time_ms == 0.0 {
            self.best_metrics.translation_time_ms = self.current_metrics.translation_time_ms;
        }

        // עדכון מדדים גרועים
        if self.current_metrics.bleu_score < self.worst_metrics.bleu_score || self.worst_metrics.bleu_score == 0.0 {
            self.worst_metrics.bleu_score = self.current_metrics.bleu_score;
        }
        if self.current_metrics.model_confidence < self.worst_metrics.model_confidence || self.worst_metrics.model_confidence == 0.0 {
            self.worst_metrics.model_confidence = self.current_metrics.model_confidence;
        }
        if self.current_metrics.translation_time_ms > self.worst_metrics.translation_time_ms {
            self.worst_metrics.translation_time_ms = self.current_metrics.translation_time_ms;
        }
    }

    pub fn get_optimization_status(&self) -> OptimizationStatus {
        let avg_metrics = self.calculate_average_metrics();
        let improvement_ratio = self.calculate_improvement_ratio(&avg_metrics);
        
        OptimizationStatus {
            current_vs_best: self.compare_to_best(),
            current_vs_average: self.compare_to_average(&avg_metrics),
            improvement_trend: improvement_ratio,
            is_optimal: self.is_optimal(&avg_metrics),
        }
    }

    fn calculate_average_metrics(&self) -> PerformanceMetrics {
        let mut avg = PerformanceMetrics::default();
        let len = self.history.len() as f64;
        
        if len == 0.0 {
            return avg;
        }

        for metrics in &self.history {
            avg.bleu_score += metrics.bleu_score;
            avg.model_confidence += metrics.model_confidence;
            avg.translation_time_ms += metrics.translation_time_ms;
            // ... המשך עבור שאר המדדים
        }

        avg.bleu_score /= len;
        avg.model_confidence /= len;
        avg.translation_time_ms /= len;
        
        avg
    }

    fn calculate_improvement_ratio(&self, avg_metrics: &PerformanceMetrics) -> f64 {
        if self.history.len() < 2 {
            return 1.0;
        }

        let recent_avg = self.calculate_recent_average(10); // ממוצע 10 המדידות האחרונות
        
        let bleu_improvement = recent_avg.bleu_score / avg_metrics.bleu_score;
        let confidence_improvement = recent_avg.model_confidence / avg_metrics.model_confidence;
        let time_improvement = avg_metrics.translation_time_ms / recent_avg.translation_time_ms;
        
        (bleu_improvement + confidence_improvement + time_improvement) / 3.0
    }

    fn is_optimal(&self, avg_metrics: &PerformanceMetrics) -> bool {
        let improvement_ratio = self.calculate_improvement_ratio(avg_metrics);
        let stable_performance = self.check_performance_stability();
        
        // מערכת נחשבת אופטימלית אם:
        // 1. שיפור ביחס לממוצע קטן מ-1%
        // 2. הביצועים יציבים לאורך זמן
        // 3. קרובה למדדים הטובים ביותר שנצפו
        
        improvement_ratio < 1.01 && 
        stable_performance && 
        self.is_close_to_best()
    }

    fn check_performance_stability(&self) -> bool {
        if self.history.len() < 10 {
            return false;
        }

        let recent = self.calculate_recent_average(10);
        let older = self.calculate_older_average(10);
        
        let bleu_diff = (recent.bleu_score - older.bleu_score).abs();
        let confidence_diff = (recent.model_confidence - older.model_confidence).abs();
        let time_diff = (recent.translation_time_ms - older.translation_time_ms).abs() / older.translation_time_ms;
        
        bleu_diff < 0.01 && confidence_diff < 0.01 && time_diff < 0.05
    }

    fn is_close_to_best(&self) -> bool {
        let current = &self.current_metrics;
        let best = &self.best_metrics;
        
        (current.bleu_score / best.bleu_score) > 0.95 &&
        (current.model_confidence / best.model_confidence) > 0.95 &&
        (best.translation_time_ms / current.translation_time_ms) > 0.95
    }
}

#[derive(Debug)]
pub struct OptimizationStatus {
    pub current_vs_best: f64,    // יחס ביצועים נוכחיים לעומת הטובים ביותר
    pub current_vs_average: f64,  // יחס ביצועים נוכחיים לעומת הממוצע
    pub improvement_trend: f64,   // מגמת שיפור לאורך זמן
    pub is_optimal: bool,         // האם המערכת במצב אופטימלי
} 