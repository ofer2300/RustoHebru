/// ספי הצלחה למדדי ביצוע
#[derive(Debug, Clone)]
pub struct PerformanceThresholds {
    pub max_translation_time_ms: u64,
    pub max_memory_usage_mb: u64,
    pub max_cpu_usage_percent: f32,
    pub min_bleu_score: f32,
    pub min_rouge_score: f32,
    pub min_meteor_score: f32,
    pub min_model_confidence: f32,
    pub max_error_rate: f32,
    pub min_success_rate: f32,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_translation_time_ms: 1000,  // 1 second
            max_memory_usage_mb: 512,       // 512 MB
            max_cpu_usage_percent: 80.0,    // 80%
            min_bleu_score: 0.7,           // 70%
            min_rouge_score: 0.7,          // 70%
            min_meteor_score: 0.7,         // 70%
            min_model_confidence: 0.8,     // 80%
            max_error_rate: 0.1,           // 10%
            min_success_rate: 0.9,         // 90%
        }
    }
}

impl PerformanceThresholds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_translation_time(mut self, time_ms: u64) -> Self {
        self.max_translation_time_ms = time_ms;
        self
    }

    pub fn with_max_memory_usage(mut self, memory_mb: u64) -> Self {
        self.max_memory_usage_mb = memory_mb;
        self
    }

    pub fn with_max_cpu_usage(mut self, cpu_percent: f32) -> Self {
        self.max_cpu_usage_percent = cpu_percent;
        self
    }

    pub fn with_min_bleu_score(mut self, bleu: f32) -> Self {
        self.min_bleu_score = bleu;
        self
    }

    pub fn with_min_rouge_score(mut self, rouge: f32) -> Self {
        self.min_rouge_score = rouge;
        self
    }

    pub fn with_min_meteor_score(mut self, meteor: f32) -> Self {
        self.min_meteor_score = meteor;
        self
    }

    pub fn with_min_model_confidence(mut self, confidence: f32) -> Self {
        self.min_model_confidence = confidence;
        self
    }

    pub fn with_max_error_rate(mut self, error_rate: f32) -> Self {
        self.max_error_rate = error_rate;
        self
    }

    pub fn with_min_success_rate(mut self, success_rate: f32) -> Self {
        self.min_success_rate = success_rate;
        self
    }
} 