#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_metrics_timing() {
        let mut collector = MetricsCollector::new();
        
        // בדיקת מדידת זמן
        collector.start_measurement();
        thread::sleep(Duration::from_millis(100));
        let elapsed = collector.end_measurement();
        
        assert!(elapsed >= 100.0, "זמן המדידה קצר מדי");
        assert!(elapsed < 150.0, "זמן המדידה ארוך מדי");
    }

    #[test]
    fn test_bleu_score_update() {
        let mut collector = MetricsCollector::new();
        let test_score = 0.85;
        
        collector.update_bleu_score(test_score);
        assert_eq!(collector.get_metrics().bleu_score, test_score);
    }

    #[test]
    fn test_model_metrics_update() {
        let mut collector = MetricsCollector::new();
        let test_confidence = 0.95;
        let test_oov_ratio = 0.05;
        
        collector.update_model_metrics(test_confidence, test_oov_ratio);
        let metrics = collector.get_metrics();
        
        assert_eq!(metrics.model_confidence, test_confidence);
        assert_eq!(metrics.out_of_vocab_ratio, test_oov_ratio);
    }

    #[test]
    fn test_metrics_serialization() {
        let mut metrics = PerformanceMetrics::default();
        metrics.bleu_score = 0.9;
        metrics.model_confidence = 0.85;
        
        let serialized = serde_json::to_string(&metrics).unwrap();
        let deserialized: PerformanceMetrics = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(metrics.bleu_score, deserialized.bleu_score);
        assert_eq!(metrics.model_confidence, deserialized.model_confidence);
    }

    #[test]
    fn test_performance_thresholds() {
        let thresholds = PerformanceThresholds::default();
        let mut metrics = PerformanceMetrics::default();
        
        // בדיקת מקרה הצלחה
        metrics.translation_time_ms = 500.0;
        metrics.bleu_score = 0.8;
        metrics.model_confidence = 0.9;
        metrics.out_of_vocab_ratio = 0.05;
        
        match thresholds.validate_metrics(&metrics) {
            ValidationResult::Success => (),
            ValidationResult::Failure(failures) => panic!("Expected success, got failures: {:?}", failures),
        }
        
        // בדיקת מקרה כישלון
        metrics.translation_time_ms = 2000.0;
        metrics.bleu_score = 0.5;
        
        match thresholds.validate_metrics(&metrics) {
            ValidationResult::Success => panic!("Expected failures, got success"),
            ValidationResult::Failure(failures) => {
                assert!(failures.contains(&ValidationFailure::TranslationTooSlow));
                assert!(failures.contains(&ValidationFailure::LowBleuScore));
            }
        }
    }

    #[test]
    fn test_resource_thresholds() {
        let thresholds = PerformanceThresholds::default();
        let mut metrics = PerformanceMetrics::default();
        
        // בדיקת שימוש במשאבים תקין
        metrics.peak_memory_mb = 1024.0;
        metrics.gpu_utilization = Some(0.7);
        
        match thresholds.validate_metrics(&metrics) {
            ValidationResult::Success => (),
            ValidationResult::Failure(failures) => panic!("Expected success, got failures: {:?}", failures),
        }
        
        // בדיקת חריגה במשאבים
        metrics.peak_memory_mb = 3072.0;
        metrics.gpu_utilization = Some(0.95);
        
        match thresholds.validate_metrics(&metrics) {
            ValidationResult::Success => panic!("Expected failures, got success"),
            ValidationResult::Failure(failures) => {
                assert!(failures.contains(&ValidationFailure::HighMemoryUsage));
                assert!(failures.contains(&ValidationFailure::HighGpuUtilization));
            }
        }
    }

    #[test]
    fn test_model_quality_thresholds() {
        let thresholds = PerformanceThresholds::default();
        let mut metrics = PerformanceMetrics::default();
        
        // בדיקת איכות מודל תקינה
        metrics.model_confidence = 0.85;
        metrics.out_of_vocab_ratio = 0.08;
        metrics.attention_entropy = 1.5;
        
        match thresholds.validate_metrics(&metrics) {
            ValidationResult::Success => (),
            ValidationResult::Failure(failures) => panic!("Expected success, got failures: {:?}", failures),
        }
        
        // בדיקת איכות מודל נמוכה
        metrics.model_confidence = 0.6;
        metrics.out_of_vocab_ratio = 0.15;
        metrics.attention_entropy = 2.5;
        
        match thresholds.validate_metrics(&metrics) {
            ValidationResult::Success => panic!("Expected failures, got success"),
            ValidationResult::Failure(failures) => {
                assert!(failures.contains(&ValidationFailure::LowConfidence));
                assert!(failures.contains(&ValidationFailure::HighOovRate));
                assert!(failures.contains(&ValidationFailure::HighAttentionEntropy));
            }
        }
    }

    #[test]
    fn test_optimization_detection() {
        let mut collector = MetricsCollector::new();
        
        // סימולציה של מדידות טובות ויציבות
        for _ in 0..20 {
            let mut metrics = PerformanceMetrics::default();
            metrics.bleu_score = 0.95;
            metrics.model_confidence = 0.92;
            metrics.translation_time_ms = 100.0;
            collector.current_metrics = metrics;
            collector.record_metrics();
        }
        
        let status = collector.get_optimization_status();
        assert!(status.is_optimal, "המערכת אמורה להיות במצב אופטימלי");
        assert!(status.improvement_trend <= 1.01, "אין אמור להיות שיפור משמעותי");
    }

    #[test]
    fn test_non_optimal_detection() {
        let mut collector = MetricsCollector::new();
        
        // סימולציה של מדידות לא יציבות
        for i in 0..20 {
            let mut metrics = PerformanceMetrics::default();
            metrics.bleu_score = 0.7 + (i as f64 * 0.01); // שיפור מתמיד
            metrics.model_confidence = 0.8 + (i as f64 * 0.005);
            metrics.translation_time_ms = 200.0 - (i as f64 * 5.0);
            collector.current_metrics = metrics;
            collector.record_metrics();
        }
        
        let status = collector.get_optimization_status();
        assert!(!status.is_optimal, "המערכת לא אמורה להיות במצב אופטימלי");
        assert!(status.improvement_trend > 1.01, "אמור להיות שיפור משמעותי");
    }

    #[test]
    fn test_performance_stability() {
        let mut collector = MetricsCollector::new();
        
        // סימולציה של ביצועים יציבים
        for _ in 0..15 {
            let mut metrics = PerformanceMetrics::default();
            metrics.bleu_score = 0.90 + (rand::random::<f64>() * 0.02 - 0.01);
            metrics.model_confidence = 0.85 + (rand::random::<f64>() * 0.02 - 0.01);
            metrics.translation_time_ms = 150.0 + (rand::random::<f64>() * 10.0 - 5.0);
            collector.current_metrics = metrics;
            collector.record_metrics();
        }
        
        assert!(collector.check_performance_stability(), "הביצועים אמורים להיות יציבים");
    }

    #[test]
    fn test_best_metrics_tracking() {
        let mut collector = MetricsCollector::new();
        
        // הכנסת מדידה טובה
        let mut best_metrics = PerformanceMetrics::default();
        best_metrics.bleu_score = 0.98;
        best_metrics.model_confidence = 0.95;
        best_metrics.translation_time_ms = 90.0;
        collector.current_metrics = best_metrics.clone();
        collector.record_metrics();
        
        // הכנסת מדידות פחות טובות
        for _ in 0..5 {
            let mut metrics = PerformanceMetrics::default();
            metrics.bleu_score = 0.85;
            metrics.model_confidence = 0.82;
            metrics.translation_time_ms = 120.0;
            collector.current_metrics = metrics;
            collector.record_metrics();
        }
        
        assert!(collector.best_metrics.bleu_score == 0.98, "לא נשמר ציון BLEU הטוב ביותר");
        assert!(collector.best_metrics.model_confidence == 0.95, "לא נשמר ציון הביטחון הטוב ביותר");
        assert!(collector.best_metrics.translation_time_ms == 90.0, "לא נשמר זמן התרגום הטוב ביותר");
    }
} 