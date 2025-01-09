use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// מילון טכני מתקדם
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedTechnicalDictionary {
    /// מונחים טכניים
    terms: HashMap<String, TechnicalTerm>,
    /// תחומים טכניים
    domains: HashMap<String, TechnicalDomain>,
    /// מערכת למידה
    learning_system: Arc<RwLock<LearningSystem>>,
    /// מערכת המלצות
    recommendation_system: Arc<RwLock<RecommendationSystem>>,
    /// היסטוריית תרגומים
    translation_history: Arc<RwLock<TranslationHistory>>,
}

/// מונח טכני
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalTerm {
    /// מזהה ייחודי
    id: String,
    /// מונח בעברית
    term_he: String,
    /// מונח ברוסית
    term_ru: String,
    /// הגדרה בעברית
    definition_he: String,
    /// הגדרה ברוסית
    definition_ru: String,
    /// תחומים קשורים
    domains: HashSet<String>,
    /// מונחים נרדפים
    synonyms: HashSet<String>,
    /// מונחים קשורים
    related_terms: HashSet<String>,
    /// תקנים רלוונטיים
    standards: HashSet<String>,
    /// מקורות
    sources: Vec<Source>,
    /// דירוג דיוק
    accuracy_score: f64,
    /// תאריך עדכון אחרון
    last_updated: DateTime<Utc>,
}

/// תחום טכני
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalDomain {
    /// מזהה ייחודי
    id: String,
    /// שם בעברית
    name_he: String,
    /// שם ברוסית
    name_ru: String,
    /// תיאור בעברית
    description_he: String,
    /// תיאור ברוסית
    description_ru: String,
    /// תחומי משנה
    subdomains: HashSet<String>,
    /// תחום אב
    parent_domain: Option<String>,
    /// מונחים שייכים
    terms: HashSet<String>,
    /// תקנים רלוונטיים
    standards: HashSet<String>,
}

/// מקור מידע
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// סוג המקור
    source_type: SourceType,
    /// כותרת
    title: String,
    /// מחבר/ארגון
    author: String,
    /// שנת פרסום
    year: i32,
    /// קישור
    url: Option<String>,
    /// דירוג אמינות
    reliability_score: f64,
}

/// סוגי מקורות
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    /// תקן רשמי
    Standard,
    /// מאמר אקדמי
    AcademicPaper,
    /// ספר מקצועי
    TechnicalBook,
    /// מסמך רשמי
    OfficialDocument,
    /// מאגר מידע מקצועי
    TechnicalDatabase,
}

/// מערכת למידה
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSystem {
    /// מודל למידה
    model: Arc<RwLock<MLModel>>,
    /// נתוני אימון
    training_data: Vec<TrainingExample>,
    /// מטריקות ביצועים
    metrics: LearningMetrics,
    /// הגדרות אימון
    training_config: TrainingConfig,
}

/// דוגמת אימון
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingExample {
    /// מונח מקור
    source_term: String,
    /// מונח יעד
    target_term: String,
    /// הקשר
    context: String,
    /// תחום
    domain: String,
    /// דירוג דיוק
    accuracy: f64,
}

/// מערכת המלצות
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationSystem {
    /// מודל המלצות
    model: Arc<RwLock<RecommendationModel>>,
    /// היסטוריית המלצות
    history: HashMap<String, Vec<Recommendation>>,
    /// הגדרות
    config: RecommendationConfig,
}

/// המלצה
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// מזהה מונח
    term_id: String,
    /// ציון התאמה
    score: f64,
    /// סיבת ההמלצה
    reason: String,
    /// תאריך
    timestamp: DateTime<Utc>,
}

/// היסטוריית תרגומים
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationHistory {
    /// תרגומים
    translations: Vec<Translation>,
    /// סטטיסטיקות
    stats: TranslationStats,
}

/// תרגום
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    /// מונח מקור
    source_term: String,
    /// מונח יעד
    target_term: String,
    /// הקשר
    context: Option<String>,
    /// תחום
    domain: Option<String>,
    /// דירוג דיוק
    accuracy: f64,
    /// תאריך
    timestamp: DateTime<Utc>,
}

impl AdvancedTechnicalDictionary {
    pub fn new() -> Self {
        Self {
            terms: HashMap::new(),
            domains: HashMap::new(),
            learning_system: Arc::new(RwLock::new(LearningSystem::new())),
            recommendation_system: Arc::new(RwLock::new(RecommendationSystem::new())),
            translation_history: Arc::new(RwLock::new(TranslationHistory::new())),
        }
    }

    /// הוספת מונח טכני חדש
    pub async fn add_term(&mut self, term: TechnicalTerm) -> Result<(), DictionaryError> {
        // וידוא תקינות
        self.validate_term(&term)?;
        
        // עדכון מערכת הלמידה
        self.update_learning_system(&term).await?;
        
        // הוספה למילון
        self.terms.insert(term.id.clone(), term);
        
        Ok(())
    }

    /// חיפוש מונח טכני
    pub async fn find_term(&self, query: &str, context: Option<&str>) -> Result<Vec<TechnicalTerm>, DictionaryError> {
        let mut results = Vec::new();
        
        // חיפוש מדויק
        if let Some(term) = self.terms.get(query) {
            results.push(term.clone());
        }
        
        // חיפוש מתקדם
        let similar_terms = self.find_similar_terms(query, context).await?;
        results.extend(similar_terms);
        
        // מיון לפי רלוונטיות
        self.sort_by_relevance(&mut results, query, context).await?;
        
        Ok(results)
    }

    /// חיפוש מונחים דומים
    async fn find_similar_terms(&self, query: &str, context: Option<&str>) -> Result<Vec<TechnicalTerm>, DictionaryError> {
        let mut similar = Vec::new();
        
        // חיפוש לפי מונחים נרדפים
        for term in self.terms.values() {
            if term.synonyms.contains(query) {
                similar.push(term.clone());
            }
        }
        
        // חיפוש לפי הקשר
        if let Some(ctx) = context {
            let recommendations = self.recommendation_system.read().await
                .get_recommendations(query, ctx)?;
            
            for term_id in recommendations {
                if let Some(term) = self.terms.get(&term_id) {
                    similar.push(term.clone());
                }
            }
        }
        
        Ok(similar)
    }

    /// מיון לפי רלוונטיות
    async fn sort_by_relevance(&self, terms: &mut Vec<TechnicalTerm>, query: &str, context: Option<&str>) -> Result<(), DictionaryError> {
        let learning_system = self.learning_system.read().await;
        
        terms.sort_by(|a, b| {
            let a_score = learning_system.calculate_relevance_score(a, query, context);
            let b_score = learning_system.calculate_relevance_score(b, query, context);
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(())
    }

    /// עדכון מערכת הלמידה
    async fn update_learning_system(&self, term: &TechnicalTerm) -> Result<(), DictionaryError> {
        let mut learning_system = self.learning_system.write().await;
        
        // הוספת דוגמת אימון
        let example = TrainingExample {
            source_term: term.term_he.clone(),
            target_term: term.term_ru.clone(),
            context: term.domains.iter().next().unwrap_or(&String::new()).clone(),
            domain: term.domains.iter().next().unwrap_or(&String::new()).clone(),
            accuracy: term.accuracy_score,
        };
        
        learning_system.add_training_example(example)?;
        
        // אימון מחדש אם נדרש
        if learning_system.should_retrain() {
            learning_system.train().await?;
        }
        
        Ok(())
    }

    /// וידוא תקינות מונח
    fn validate_term(&self, term: &TechnicalTerm) -> Result<(), DictionaryError> {
        // בדיקת שדות חובה
        if term.term_he.is_empty() || term.term_ru.is_empty() {
            return Err(DictionaryError::ValidationError("שדות חובה חסרים".to_string()));
        }
        
        // בדיקת תחומים
        for domain in &term.domains {
            if !self.domains.contains_key(domain) {
                return Err(DictionaryError::ValidationError(format!("תחום לא קיים: {}", domain)));
            }
        }
        
        // בדיקת מונחים קשורים
        for related in &term.related_terms {
            if !self.terms.contains_key(related) {
                return Err(DictionaryError::ValidationError(format!("מונח קשור לא קיים: {}", related)));
            }
        }
        
        Ok(())
    }

    /// הוספת תחום טכני
    pub async fn add_domain(&mut self, domain: TechnicalDomain) -> Result<(), DictionaryError> {
        // וידוא תקינות
        self.validate_domain(&domain)?;
        
        // הוספה למילון
        self.domains.insert(domain.id.clone(), domain);
        
        Ok(())
    }

    /// וידוא תקינות תחום
    fn validate_domain(&self, domain: &TechnicalDomain) -> Result<(), DictionaryError> {
        // בדיקת שדות חובה
        if domain.name_he.is_empty() || domain.name_ru.is_empty() {
            return Err(DictionaryError::ValidationError("שדות חובה חסרים".to_string()));
        }
        
        // בדיקת תחום אב
        if let Some(parent) = &domain.parent_domain {
            if !self.domains.contains_key(parent) {
                return Err(DictionaryError::ValidationError(format!("תחום אב לא קיים: {}", parent)));
            }
        }
        
        Ok(())
    }

    /// עדכון היסטוריית תרגומים
    pub async fn update_translation_history(&self, translation: Translation) -> Result<(), DictionaryError> {
        let mut history = self.translation_history.write().await;
        history.add_translation(translation)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_term() {
        let mut dict = AdvancedTechnicalDictionary::new();
        
        let term = TechnicalTerm {
            id: "test1".to_string(),
            term_he: "בדיקה".to_string(),
            term_ru: "тест".to_string(),
            definition_he: "מונח לבדיקה".to_string(),
            definition_ru: "тестовый термин".to_string(),
            domains: HashSet::new(),
            synonyms: HashSet::new(),
            related_terms: HashSet::new(),
            standards: HashSet::new(),
            sources: Vec::new(),
            accuracy_score: 1.0,
            last_updated: Utc::now(),
        };
        
        assert!(dict.add_term(term).await.is_ok());
    }

    #[tokio::test]
    async fn test_find_term() {
        let dict = AdvancedTechnicalDictionary::new();
        let results = dict.find_term("test", None).await.unwrap();
        assert!(results.is_empty());
    }
}