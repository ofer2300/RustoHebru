use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use metrics::{counter, gauge, histogram};

/// מערכת ניטור מתקדמת
pub struct AdvancedMonitoringSystem {
    /// מערכת התראות
    alert_system: Arc<AlertSystem>,
    /// אוסף מדדים
    metrics_collector: Arc<MetricsCollector>,
    /// מערכת דוחות
    reporting_system: Arc<ReportingSystem>,
    /// מערכת חיזוי
    prediction_system: Arc<PredictionSystem>,
    /// מערכת התאוששות
    recovery_system: Arc<RecoverySystem>,
}

/// מערכת התראות
#[derive(Debug)]
pub struct AlertSystem {
    /// התראות פעילות
    active_alerts: Arc<RwLock<HashMap<String, Alert>>>,
    /// היסטוריית התראות
    alert_history: Arc<RwLock<VecDeque<Alert>>>,
    /// מדיניות התראות
    policies: Arc<RwLock<HashMap<String, AlertPolicy>>>,
    /// ערוצי התראות
    channels: Vec<Box<dyn AlertChannel>>,
}

/// התראה
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// מזהה ייחודי
    id: String,
    /// סוג ההתראה
    alert_type: AlertType,
    /// חומרת ההתראה
    severity: AlertSeverity,
    /// תיאור
    description: String,
    /// מקור
    source: String,
    /// תאריך יצירה
    created_at: DateTime<Utc>,
    /// תאריך עדכון אחרון
    updated_at: DateTime<Utc>,
    /// סטטוס
    status: AlertStatus,
    /// נתונים נוספים
    metadata: HashMap<String, String>,
}

/// סוג התראה
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    /// ביצועים
    Performance {
        metric: String,
        threshold: f64,
        current_value: f64,
    },
    /// זמינות
    Availability {
        service: String,
        status: ServiceStatus,
    },
    /// אבטחה
    Security {
        threat_type: String,
        risk_level: RiskLevel,
    },
    /// שגיאות
    Error {
        error_type: String,
        error_count: u32,
    },
    /// משאבים
    Resource {
        resource_type: String,
        usage_percent: f64,
    },
}

/// חומרת התראה
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    /// נמוכה
    Low,
    /// בינונית
    Medium,
    /// גבוהה
    High,
    /// קריטית
    Critical,
}

/// סטטוס התראה
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertStatus {
    /// פעילה
    Active,
    /// בטיפול
    Acknowledged {
        by: String,
        at: DateTime<Utc>,
    },
    /// נפתרה
    Resolved {
        by: String,
        at: DateTime<Utc>,
        resolution: String,
    },
}

/// מדיניות התראות
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertPolicy {
    /// שם המדיניות
    name: String,
    /// תנאים
    conditions: Vec<AlertCondition>,
    /// פעולות
    actions: Vec<AlertAction>,
    /// הגדרות
    settings: AlertPolicySettings,
}

/// תנאי התראה
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    /// סוג המדד
    metric_type: String,
    /// מפעיל
    operator: ComparisonOperator,
    /// ערך סף
    threshold: f64,
    /// משך זמן
    duration: std::time::Duration,
}

/// פעולת התראה
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertAction {
    /// סוג הפעולה
    action_type: ActionType,
    /// יעד
    target: String,
    /// תבנית הודעה
    message_template: String,
}

impl AdvancedMonitoringSystem {
    pub fn new() -> Self {
        Self {
            alert_system: Arc::new(AlertSystem::new()),
            metrics_collector: Arc::new(MetricsCollector::new()),
            reporting_system: Arc::new(ReportingSystem::new()),
            prediction_system: Arc::new(PredictionSystem::new()),
            recovery_system: Arc::new(RecoverySystem::new()),
        }
    }

    /// התחלת ניטור
    pub async fn start_monitoring(&self) -> Result<(), MonitoringError> {
        // התחלת איסוף מדדים
        self.metrics_collector.start().await?;
        
        // הפעלת מערכת התראות
        self.alert_system.start().await?;
        
        // הפעלת מערכת חיזוי
        self.prediction_system.start().await?;
        
        // הפעלת מערכת התאוששות
        self.recovery_system.start().await?;
        
        Ok(())
    }

    /// הוספת התראה
    pub async fn add_alert(&self, alert: Alert) -> Result<(), MonitoringError> {
        // עדכון מדדים
        self.update_alert_metrics(&alert);
        
        // הוספת ההתראה
        self.alert_system.add_alert(alert.clone()).await?;
        
        // בדיקת מדיניות
        self.check_alert_policies(&alert).await?;
        
        Ok(())
    }

    /// עדכון מדדי התראות
    fn update_alert_metrics(&self, alert: &Alert) {
        // עדכון מונה התראות לפי סוג
        counter!("alerts_total", 1, "type" => alert.alert_type.to_string());
        
        // עדכון מונה התראות לפי חומרה
        counter!("alerts_by_severity", 1, "severity" => alert.severity.to_string());
        
        // עדכון זמן טיפול
        if let AlertStatus::Resolved { at, .. } = alert.status {
            let duration = at.signed_duration_since(alert.created_at);
            histogram!("alert_resolution_time", duration.num_seconds() as f64);
        }
    }

    /// בדיקת מדיניות התראות
    async fn check_alert_policies(&self, alert: &Alert) -> Result<(), MonitoringError> {
        let policies = self.alert_system.get_policies().await?;
        
        for policy in policies.values() {
            if self.should_apply_policy(policy, alert) {
                self.apply_policy_actions(policy, alert).await?;
            }
        }
        
        Ok(())
    }

    /// בדיקה האם להפעיל מדיניות
    fn should_apply_policy(&self, policy: &AlertPolicy, alert: &Alert) -> bool {
        // בדיקת תנאי סף
        for condition in &policy.conditions {
            if !self.check_condition(condition, alert) {
                return false;
            }
        }
        
        true
    }

    /// בדיקת תנאי
    fn check_condition(&self, condition: &AlertCondition, alert: &Alert) -> bool {
        match &alert.alert_type {
            AlertType::Performance { metric, current_value, .. } => {
                if metric == &condition.metric_type {
                    match condition.operator {
                        ComparisonOperator::GreaterThan => *current_value > condition.threshold,
                        ComparisonOperator::LessThan => *current_value < condition.threshold,
                        ComparisonOperator::Equals => (*current_value - condition.threshold).abs() < f64::EPSILON,
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// הפעלת פעולות מדיניות
    async fn apply_policy_actions(&self, policy: &AlertPolicy, alert: &Alert) -> Result<(), MonitoringError> {
        for action in &policy.actions {
            match action.action_type {
                ActionType::Notification => {
                    self.send_notification(action, alert).await?;
                }
                ActionType::Remediation => {
                    self.apply_remediation(action, alert).await?;
                }
                ActionType::Escalation => {
                    self.escalate_alert(action, alert).await?;
                }
            }
        }
        
        Ok(())
    }
}

impl AlertSystem {
    pub fn new() -> Self {
        Self {
            active_alerts: Arc::new(RwLock::new(HashMap::new())),
            alert_history: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            policies: Arc::new(RwLock::new(HashMap::new())),
            channels: Vec::new(),
        }
    }

    /// הוספת התראה
    pub async fn add_alert(&self, alert: Alert) -> Result<(), MonitoringError> {
        // הוספה להתראות פעילות
        self.active_alerts.write().await.insert(alert.id.clone(), alert.clone());
        
        // הוספה להיסטוריה
        let mut history = self.alert_history.write().await;
        history.push_back(alert.clone());
        
        // שמירה על גודל מקסימלי
        while history.len() > 1000 {
            history.pop_front();
        }
        
        // שליחת התראה בכל הערוצים
        for channel in &self.channels {
            channel.send_alert(&alert).await?;
        }
        
        Ok(())
    }

    /// קבלת מדיניות התראות
    pub async fn get_policies(&self) -> Result<HashMap<String, AlertPolicy>, MonitoringError> {
        Ok(self.policies.read().await.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_alert() {
        let monitoring = AdvancedMonitoringSystem::new();
        
        let alert = Alert {
            id: "test1".to_string(),
            alert_type: AlertType::Performance {
                metric: "response_time".to_string(),
                threshold: 1.0,
                current_value: 2.0,
            },
            severity: AlertSeverity::High,
            description: "High response time".to_string(),
            source: "api_server".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: AlertStatus::Active,
            metadata: HashMap::new(),
        };
        
        assert!(monitoring.add_alert(alert).await.is_ok());
    }
} 