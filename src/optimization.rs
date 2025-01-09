use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::{HashMap, BTreeMap};
use chrono::{DateTime, Utc};
use dashmap::DashMap;

/// מנהל אופטימיזציה משופר
pub struct OptimizationManager {
    /// מנהל זיכרון מטמון היברידי
    cache_manager: Arc<HybridCacheManager>,
    /// מנהל עומסים מתקדם
    load_balancer: Arc<AdvancedLoadBalancer>,
    /// מערכת מדידה מורחבת
    metrics: Arc<EnhancedMetricsCollector>,
    /// מערכת למידה מתקדמת
    learning: Arc<AdvancedMachineLearning>,
}

/// מנהל זיכרון מטמון היברידי משופר
pub struct HybridCacheManager {
    /// מטמון L1 (זיכרון מהיר)
    l1_cache: Arc<DashMap<String, CacheEntry>>,
    /// מטמון L2 (זיכרון ראשי)
    l2_cache: Arc<Mutex<BTreeMap<String, CacheEntry>>>,
    /// מטמון L3 (דיסק)
    l3_cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    /// מדיניות מטמון מתקדמת
    policy: AdvancedCachePolicy,
    /// מנגנון דחיסה
    compression: Arc<CompressionEngine>,
}

/// רשומת מטמון משופרת
#[derive(Clone)]
struct CacheEntry {
    /// ערך מקודד
    value: Vec<u8>,
    /// תאריך יצירה
    created: DateTime<Utc>,
    /// תאריך גישה אחרון
    last_accessed: DateTime<Utc>,
    /// מספר גישות
    access_count: u64,
    /// רמת חשיבות
    priority: u8,
    /// גודל מקורי
    original_size: usize,
    /// גודל דחוס
    compressed_size: usize,
}

/// מדיניות מטמון מתקדמת
struct AdvancedCachePolicy {
    /// זמן תפוגה מקסימלי
    max_age_seconds: i64,
    /// מספר רשומות מקסימלי
    max_entries: usize,
    /// כמות רשומות לפינוי
    eviction_count: usize,
    /// סף ניצול זיכרון
    memory_threshold: f64,
    /// מדיניות החלפה
    replacement_strategy: ReplacementStrategy,
}

/// אסטרטגיות החלפת מטמון
enum ReplacementStrategy {
    /// Least Recently Used
    LRU,
    /// Most Recently Used
    MRU,
    /// Least Frequently Used
    LFU,
    /// Adaptive Replacement Cache
    ARC,
    /// Time Aware Least Recently Used
    TLRU,
}

impl HybridCacheManager {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            l1_cache: Arc::new(DashMap::new()),
            l2_cache: Arc::new(Mutex::new(BTreeMap::new())),
            l3_cache: Arc::new(Mutex::new(HashMap::new())),
            policy: AdvancedCachePolicy {
                max_age_seconds: 3600,
                max_entries: 10000,
                eviction_count: 100,
                memory_threshold: 0.85,
                replacement_strategy: ReplacementStrategy::ARC,
            },
            compression: Arc::new(CompressionEngine::new()),
        }
    }

    /// קבלת ערך ממטמון
    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        // בדיקה במטמון L1
        if let Some(entry) = self.l1_cache.get(key) {
            self.update_metrics(key, &entry, CacheLevel::L1).await;
            return Some(self.decompress_value(&entry.value));
        }

        // בדיקה במטמון L2
        let l2_cache = self.l2_cache.lock().await;
        if let Some(entry) = l2_cache.get(key) {
            self.promote_to_l1(key, entry.clone()).await;
            self.update_metrics(key, entry, CacheLevel::L2).await;
            return Some(self.decompress_value(&entry.value));
        }

        // בדיקה במטמון L3
        let l3_cache = self.l3_cache.lock().await;
        if let Some(entry) = l3_cache.get(key) {
            self.promote_to_l2(key, entry.clone()).await;
            self.update_metrics(key, entry, CacheLevel::L3).await;
            return Some(self.decompress_value(&entry.value));
        }

        None
    }

    /// שמירת ערך במטמון
    pub async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), CacheError> {
        let compressed_value = self.compress_value(&value);
        let entry = CacheEntry {
            value: compressed_value,
            created: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 1,
            priority: self.calculate_priority(key, &value),
            original_size: value.len(),
            compressed_size: compressed_value.len(),
        };

        // שמירה במטמון L1
        self.l1_cache.insert(key.to_string(), entry.clone());

        // פינוי מטמון אם נדרש
        self.evict_if_needed().await?;

        Ok(())
    }

    /// חישוב עדיפות לרשומה
    fn calculate_priority(&self, key: &str, value: &[u8]) -> u8 {
        let mut priority = 0;
        
        // עדיפות גבוהה למונחים טכניים
        if key.contains("technical_") {
            priority += 2;
        }
        
        // עדיפות לפי גודל
        if value.len() < 1024 {
            priority += 1;
        }
        
        priority
    }

    /// פינוי רשומות לפי צורך
    async fn evict_if_needed(&self) -> Result<(), CacheError> {
        let memory_usage = self.get_memory_usage().await;
        
        if memory_usage > self.policy.memory_threshold {
            match self.policy.replacement_strategy {
                ReplacementStrategy::ARC => self.evict_arc().await?,
                ReplacementStrategy::TLRU => self.evict_tlru().await?,
                _ => self.evict_lru().await?,
            }
        }
        
        Ok(())
    }

    /// פינוי בשיטת ARC
    async fn evict_arc(&self) -> Result<(), CacheError> {
        let mut l1_cache = self.l1_cache.clone();
        let now = Utc::now();
        
        // מיון לפי שימוש ועדיפות
        let mut entries: Vec<_> = l1_cache.iter().collect();
        entries.sort_by(|a, b| {
            let a_score = a.access_count as f64 * a.priority as f64;
            let b_score = b.access_count as f64 * b.priority as f64;
            b_score.partial_cmp(&a_score).unwrap()
        });
        
        // פינוי הרשומות הפחות שימושיות
        for entry in entries.iter().take(self.policy.eviction_count) {
            l1_cache.remove(entry.key());
        }
        
        Ok(())
    }
}

/// מנהל עומסים מתקדם
pub struct AdvancedLoadBalancer {
    /// רשימת שרתים
    servers: Arc<DashMap<String, ServerInfo>>,
    /// היסטוריית עומסים
    load_history: Arc<Mutex<BTreeMap<DateTime<Utc>, HashMap<String, f64>>>>,
    /// מנגנון חיזוי
    predictor: Arc<LoadPredictor>,
    /// מדיניות איזון
    policy: BalancingPolicy,
    /// מטריקות ביצועים
    metrics: Arc<LoadBalancerMetrics>,
}

/// מידע על שרת
#[derive(Clone)]
struct ServerInfo {
    /// כתובת השרת
    address: String,
    /// עומס נוכחי
    current_load: f64,
    /// זמן תגובה ממוצע
    avg_response_time: f64,
    /// זמינות
    availability: f64,
    /// משאבים זמינים
    resources: ServerResources,
    /// סטטיסטיקות
    stats: ServerStats,
}

/// משאבי שרת
#[derive(Clone)]
struct ServerResources {
    /// ניצול מעבד
    cpu_usage: f64,
    /// ניצול זיכרון
    memory_usage: f64,
    /// ניצול דיסק
    disk_usage: f64,
    /// רוחב פס זמין
    bandwidth: f64,
}

/// סטטיסטיקות שרת
#[derive(Clone)]
struct ServerStats {
    /// מספר בקשות פעילות
    active_requests: u64,
    /// זמן תגובה מינימלי
    min_response_time: f64,
    /// זמן תגובה מקסימלי
    max_response_time: f64,
    /// שגיאות בדקה האחרונה
    errors_per_minute: u32,
}

/// מדיניות איזון עומסים
#[derive(Clone)]
enum BalancingPolicy {
    /// Round Robin משופר
    WeightedRoundRobin {
        weights: HashMap<String, f64>,
    },
    /// Least Connections משופר
    WeightedLeastConnections {
        max_connections: u32,
    },
    /// Resource Based
    ResourceBased {
        cpu_weight: f64,
        memory_weight: f64,
        response_time_weight: f64,
    },
    /// Machine Learning Based
    MachineLearning {
        model: Arc<MLModel>,
    },
}

impl AdvancedLoadBalancer {
    pub fn new(config: LoadBalancerConfig) -> Self {
        Self {
            servers: Arc::new(DashMap::new()),
            load_history: Arc::new(Mutex::new(BTreeMap::new())),
            predictor: Arc::new(LoadPredictor::new()),
            policy: config.policy,
            metrics: Arc::new(LoadBalancerMetrics::new()),
        }
    }

    /// בחירת שרת אופטימלי
    pub async fn select_server(&self, request: &Request) -> Result<String, LoadBalancerError> {
        // עדכון מטריקות
        self.update_metrics().await?;
        
        // חיזוי עומסים
        let predictions = self.predictor.predict_loads().await?;
        
        // בחירת שרת לפי מדיניות
        let selected = match &self.policy {
            BalancingPolicy::WeightedRoundRobin { weights } => {
                self.select_weighted_round_robin(weights).await?
            }
            BalancingPolicy::WeightedLeastConnections { max_connections } => {
                self.select_weighted_least_connections(*max_connections).await?
            }
            BalancingPolicy::ResourceBased { cpu_weight, memory_weight, response_time_weight } => {
                self.select_resource_based(*cpu_weight, *memory_weight, *response_time_weight).await?
            }
            BalancingPolicy::MachineLearning { model } => {
                self.select_ml_based(model, request).await?
            }
        };
        
        // עדכון סטטיסטיקות
        self.update_server_stats(&selected).await?;
        
        Ok(selected)
    }

    /// בחירת שרת בשיטת Weighted Round Robin
    async fn select_weighted_round_robin(&self, weights: &HashMap<String, f64>) -> Result<String, LoadBalancerError> {
        let mut best_server = None;
        let mut best_score = f64::NEG_INFINITY;
        
        for server in self.servers.iter() {
            let weight = weights.get(server.key()).unwrap_or(&1.0);
            let load_factor = 1.0 - server.current_load;
            let availability_factor = server.availability;
            
            let score = weight * load_factor * availability_factor;
            
            if score > best_score {
                best_score = score;
                best_server = Some(server.key().clone());
            }
        }
        
        best_server.ok_or(LoadBalancerError::NoServersAvailable)
    }

    /// בחירת שרת בשיטת Weighted Least Connections
    async fn select_weighted_least_connections(&self, max_connections: u32) -> Result<String, LoadBalancerError> {
        let mut best_server = None;
        let mut min_connections = u32::MAX;
        
        for server in self.servers.iter() {
            let active_connections = server.stats.active_requests as u32;
            
            if active_connections < max_connections && active_connections < min_connections {
                min_connections = active_connections;
                best_server = Some(server.key().clone());
            }
        }
        
        best_server.ok_or(LoadBalancerError::NoServersAvailable)
    }

    /// בחירת שרת לפי משאבים
    async fn select_resource_based(
        &self,
        cpu_weight: f64,
        memory_weight: f64,
        response_time_weight: f64
    ) -> Result<String, LoadBalancerError> {
        let mut best_server = None;
        let mut best_score = f64::NEG_INFINITY;
        
        for server in self.servers.iter() {
            let cpu_score = (1.0 - server.resources.cpu_usage) * cpu_weight;
            let memory_score = (1.0 - server.resources.memory_usage) * memory_weight;
            let response_score = (1.0 / server.avg_response_time) * response_time_weight;
            
            let total_score = cpu_score + memory_score + response_score;
            
            if total_score > best_score {
                best_score = total_score;
                best_server = Some(server.key().clone());
            }
        }
        
        best_server.ok_or(LoadBalancerError::NoServersAvailable)
    }

    /// בחירת שרת באמצעות למידת מכונה
    async fn select_ml_based(&self, model: &MLModel, request: &Request) -> Result<String, LoadBalancerError> {
        let features = self.extract_features(request);
        let predictions = model.predict(&features).await?;
        
        let mut best_server = None;
        let mut best_score = f64::NEG_INFINITY;
        
        for (server, score) in predictions {
            if score > best_score {
                best_score = score;
                best_server = Some(server);
            }
        }
        
        best_server.ok_or(LoadBalancerError::NoServersAvailable)
    }

    /// עדכון מטריקות שרת
    async fn update_server_stats(&self, server_id: &str) -> Result<(), LoadBalancerError> {
        if let Some(mut server) = self.servers.get_mut(server_id) {
            // עדכון סטטיסטיקות
            server.stats.active_requests += 1;
            
            // שמירת היסטוריה
            let mut history = self.load_history.lock().await;
            let current_time = Utc::now();
            
            let mut loads = HashMap::new();
            loads.insert(server_id.to_string(), server.current_load);
            history.insert(current_time, loads);
            
            // מחיקת היסטוריה ישנה
            history.retain(|time, _| {
                current_time.signed_duration_since(*time).num_minutes() < 60
            });
        }
        
        Ok(())
    }
}

/// אוסף מדדים מורחב
pub struct EnhancedMetricsCollector {
    /// מדדי ביצועים
    performance: Arc<DashMap<String, PerformanceMetrics>>,
    /// מדדי דיוק
    accuracy: Arc<DashMap<String, AccuracyMetrics>>,
    /// מדדי משאבים
    resources: Arc<DashMap<String, ResourceMetrics>>,
    /// מדדי אבטחה
    security: Arc<DashMap<String, SecurityMetrics>>,
    /// מערכת התראות
    alerts: Arc<AlertSystem>,
    /// היסטוריית מדדים
    history: Arc<MetricsHistory>,
}

/// מדדי ביצועים
#[derive(Clone)]
struct PerformanceMetrics {
    /// זמן תגובה ממוצע
    avg_response_time: f64,
    /// זמן עיבוד ממוצע
    avg_processing_time: f64,
    /// מספר בקשות לשנייה
    requests_per_second: f64,
    /// שיעור הצלחה
    success_rate: f64,
    /// זמן השהייה
    latency: LatencyMetrics,
}

/// מדדי זמן השהייה
#[derive(Clone)]
struct LatencyMetrics {
    /// זמן השהייה מינימלי
    min: f64,
    /// זמן השהייה מקסימלי
    max: f64,
    /// אחוזון 50
    p50: f64,
    /// אחוזון 90
    p90: f64,
    /// אחוזון 99
    p99: f64,
}

/// מדדי דיוק
#[derive(Clone)]
struct AccuracyMetrics {
    /// דיוק תרגום
    translation_accuracy: f64,
    /// דיוק זיהוי שפה
    language_detection_accuracy: f64,
    /// דיוק זיהוי מונחים טכניים
    technical_terms_accuracy: f64,
    /// מטריצת בלבול
    confusion_matrix: ConfusionMatrix,
}

/// מטריצת בלבול
#[derive(Clone)]
struct ConfusionMatrix {
    /// True Positives
    true_positives: u64,
    /// True Negatives
    true_negatives: u64,
    /// False Positives
    false_positives: u64,
    /// False Negatives
    false_negatives: u64,
}

/// מדדי משאבים
#[derive(Clone)]
struct ResourceMetrics {
    /// ניצול מעבד
    cpu: CpuMetrics,
    /// ניצול זיכרון
    memory: MemoryMetrics,
    /// ניצול דיסק
    disk: DiskMetrics,
    /// ניצול רשת
    network: NetworkMetrics,
}

/// מדדי מעבד
#[derive(Clone)]
struct CpuMetrics {
    /// אחוז ניצול
    usage_percent: f64,
    /// מספר ליבות בשימוש
    cores_used: u32,
    /// טמפרטורה
    temperature: f64,
    /// תדר
    frequency: f64,
}

/// מדדי זיכרון
#[derive(Clone)]
struct MemoryMetrics {
    /// זיכרון בשימוש
    used_bytes: u64,
    /// זיכרון פנוי
    free_bytes: u64,
    /// זיכרון מטמון
    cached_bytes: u64,
    /// שיעור ניצול
    usage_percent: f64,
}

/// מדדי דיסק
#[derive(Clone)]
struct DiskMetrics {
    /// קצב קריאה
    read_bytes_per_second: f64,
    /// קצב כתיבה
    write_bytes_per_second: f64,
    /// זמן גישה ממוצע
    avg_access_time: f64,
    /// מקום פנוי
    free_space_bytes: u64,
}

/// מדדי רשת
#[derive(Clone)]
struct NetworkMetrics {
    /// קצב קבלת נתונים
    bytes_received_per_second: f64,
    /// קצב שליחת נתונים
    bytes_sent_per_second: f64,
    /// מספר חיבורים פעילים
    active_connections: u32,
    /// זמן השהייה ברשת
    network_latency: f64,
}

/// מדדי אבטחה
#[derive(Clone)]
struct SecurityMetrics {
    /// ניסיונות גישה חשודים
    suspicious_attempts: u32,
    /// בקשות חסומות
    blocked_requests: u32,
    /// דירוג אבטחה
    security_score: f64,
    /// זמן תגובה לאיומים
    threat_response_time: f64,
}

impl EnhancedMetricsCollector {
    pub fn new() -> Self {
        Self {
            performance: Arc::new(DashMap::new()),
            accuracy: Arc::new(DashMap::new()),
            resources: Arc::new(DashMap::new()),
            security: Arc::new(DashMap::new()),
            alerts: Arc::new(AlertSystem::new()),
            history: Arc::new(MetricsHistory::new()),
        }
    }

    /// איסוף מדדים
    pub async fn collect_metrics(&self) -> Result<(), MetricsError> {
        // איסוף מדדי ביצועים
        self.collect_performance_metrics().await?;
        
        // איסוף מדדי דיוק
        self.collect_accuracy_metrics().await?;
        
        // איסוף מדדי משאבים
        self.collect_resource_metrics().await?;
        
        // איסוף מדדי אבטחה
        self.collect_security_metrics().await?;
        
        // שמירת היסטוריה
        self.update_history().await?;
        
        // בדיקת התראות
        self.check_alerts().await?;
        
        Ok(())
    }

    /// איסוף מדדי ביצועים
    async fn collect_performance_metrics(&self) -> Result<(), MetricsError> {
        for server_id in self.get_active_servers().await? {
            let metrics = self.measure_performance(&server_id).await?;
            self.performance.insert(server_id.clone(), metrics);
            
            // בדיקת חריגות
            if metrics.avg_response_time > 1.0 || metrics.success_rate < 0.99 {
                self.alerts.trigger_alert(
                    AlertType::Performance,
                    &format!("ביצועים ירודים בשרת {}", server_id),
                    AlertSeverity::Warning,
                ).await?;
            }
        }
        
        Ok(())
    }

    /// מדידת ביצועים
    async fn measure_performance(&self, server_id: &str) -> Result<PerformanceMetrics, MetricsError> {
        let latency = self.measure_latency(server_id).await?;
        
        Ok(PerformanceMetrics {
            avg_response_time: self.calculate_avg_response_time(server_id).await?,
            avg_processing_time: self.calculate_avg_processing_time(server_id).await?,
            requests_per_second: self.calculate_requests_per_second(server_id).await?,
            success_rate: self.calculate_success_rate(server_id).await?,
            latency,
        })
    }

    /// מדידת זמני השהייה
    async fn measure_latency(&self, server_id: &str) -> Result<LatencyMetrics, MetricsError> {
        let latencies = self.collect_latency_samples(server_id).await?;
        
        Ok(LatencyMetrics {
            min: latencies.iter().copied().fold(f64::INFINITY, f64::min),
            max: latencies.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            p50: self.calculate_percentile(&latencies, 0.5)?,
            p90: self.calculate_percentile(&latencies, 0.9)?,
            p99: self.calculate_percentile(&latencies, 0.99)?,
        })
    }

    /// חישוב אחוזון
    fn calculate_percentile(&self, values: &[f64], percentile: f64) -> Result<f64, MetricsError> {
        if values.is_empty() {
            return Err(MetricsError::NoDataAvailable);
        }
        
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let index = (percentile * (sorted.len() - 1) as f64).round() as usize;
        Ok(sorted[index])
    }
}

impl OptimizationManager {
    pub async fn new() -> Self {
        Self {
            cache_manager: Arc::new(CacheManager::new()),
            load_balancer: Arc::new(LoadBalancer::new()),
            metrics: Arc::new(MetricsCollector::new()),
            learning: Arc::new(MachineLearning::new()),
        }
    }

    /// מטמן ערך
    pub async fn cache_value(&self, key: &str, value: Vec<u8>) -> Result<(), OptimizationError> {
        self.cache_manager.set(key, value).await
    }

    /// מקבל ערך ממטמון
    pub async fn get_cached(&self, key: &str) -> Option<Vec<u8>> {
        self.cache_manager.get(key).await
    }

    /// מאזן עומסים
    pub async fn balance_load(&self) -> Result<(), OptimizationError> {
        self.load_balancer.balance().await
    }

    /// אוסף מדדים
    pub async fn collect_metrics(&self) -> Result<(), OptimizationError> {
        self.metrics.collect().await
    }

    /// מאמן מודל למידה
    pub async fn train_model(&self, data: &[TrainingData]) -> Result<(), OptimizationError> {
        self.learning.train(data).await
    }
}

impl LoadBalancer {
    pub fn new() -> Self {
        Self {
            loads: Arc::new(Mutex::new(HashMap::new())),
            threshold: 0.8,
        }
    }

    /// מאזן עומסים
    pub async fn balance(&self) -> Result<(), OptimizationError> {
        let mut loads = self.loads.lock().await;
        
        // חישוב עומס ממוצע
        let avg_load: f64 = loads.values().sum::<f64>() / loads.len() as f64;
        
        // איזון עומסים
        for (_, load) in loads.iter_mut() {
            if *load > self.threshold {
                *load = avg_load;
            }
        }
        
        Ok(())
    }
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            performance: Arc::new(Mutex::new(PerformanceMetrics::default())),
            accuracy: Arc::new(Mutex::new(AccuracyMetrics::default())),
            resources: Arc::new(Mutex::new(ResourceMetrics::default())),
        }
    }

    /// אוסף מדדים
    pub async fn collect(&self) -> Result<(), OptimizationError> {
        // איסוף מדדי ביצועים
        let mut performance = self.performance.lock().await;
        performance.collect().await?;
        
        // איסוף מדדי דיוק
        let mut accuracy = self.accuracy.lock().await;
        accuracy.collect().await?;
        
        // איסוף מדדי משאבים
        let mut resources = self.resources.lock().await;
        resources.collect().await?;
        
        Ok(())
    }
} 