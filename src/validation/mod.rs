#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
    pub confidence_score: f64,
}

impl ValidationReport {
    pub fn new(is_valid: bool, confidence_score: f64) -> Self {
        Self {
            is_valid,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
            confidence_score,
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    pub fn update_confidence(&mut self, new_score: f64) {
        self.confidence_score = new_score;
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn has_suggestions(&self) -> bool {
        !self.suggestions.is_empty()
    }
} 