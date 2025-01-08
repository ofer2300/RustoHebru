use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, Duration};
use anyhow::{Result, anyhow};
use rand::Rng;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityToken {
    pub token: String,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub user_id: Option<String>,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    Upload,
    Download,
    Translate,
    Edit,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileValidationResult {
    pub is_valid: bool,
    pub file_type: String,
    pub file_size: u64,
    pub validation_errors: Vec<String>,
    pub security_warnings: Vec<String>,
    pub virus_scan_result: Option<VirusScanResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirusScanResult {
    pub is_clean: bool,
    pub threats_found: Vec<String>,
    pub scan_date: SystemTime,
}

pub struct SecurityManager {
    active_tokens: Arc<RwLock<HashMap<String, SecurityToken>>>,
    file_validators: Vec<Box<dyn FileValidator + Send + Sync>>,
    max_file_size: u64,
    allowed_extensions: Vec<String>,
    blocked_ips: Arc<Mutex<HashMap<String, SystemTime>>>,
    rate_limits: Arc<Mutex<HashMap<String, Vec<SystemTime>>>>,
    encryption_key: [u8; 32],
}

pub trait FileValidator: Send + Sync {
    fn validate(&self, path: &Path) -> Result<FileValidationResult>;
}

impl SecurityManager {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut key = [0u8; 32];
        rng.fill(&mut key);
        
        let mut allowed_extensions = Vec::new();
        allowed_extensions.extend(vec![
            "pdf".to_string(),
            "doc".to_string(), "docx".to_string(),
            "xls".to_string(), "xlsx".to_string(),
            "ppt".to_string(), "pptx".to_string(),
            "html".to_string(), "htm".to_string(),
            "csv".to_string(),
            "txt".to_string(),
        ]);
        
        Self {
            active_tokens: Arc::new(RwLock::new(HashMap::new())),
            file_validators: Vec::new(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            allowed_extensions,
            blocked_ips: Arc::new(Mutex::new(HashMap::new())),
            rate_limits: Arc::new(Mutex::new(HashMap::new())),
            encryption_key: key,
        }
    }
    
    pub async fn generate_token(&self, user_id: Option<String>, permissions: Vec<Permission>) -> Result<SecurityToken> {
        let mut rng = rand::thread_rng();
        let mut token_bytes = [0u8; 32];
        rng.fill(&mut token_bytes);
        
        let token = BASE64.encode(token_bytes);
        let now = SystemTime::now();
        let expires_at = now + Duration::from_secs(3600); // תוקף של שעה
        
        let security_token = SecurityToken {
            token: token.clone(),
            created_at: now,
            expires_at,
            user_id,
            permissions,
        };
        
        self.active_tokens.write().await.insert(token.clone(), security_token.clone());
        
        Ok(security_token)
    }
    
    pub async fn validate_token(&self, token: &str) -> Result<SecurityToken> {
        let tokens = self.active_tokens.read().await;
        
        if let Some(token_data) = tokens.get(token) {
            if token_data.expires_at > SystemTime::now() {
                Ok(token_data.clone())
            } else {
                Err(anyhow!("Token has expired"))
            }
        } else {
            Err(anyhow!("Invalid token"))
        }
    }
    
    pub async fn revoke_token(&self, token: &str) -> Result<()> {
        self.active_tokens.write().await.remove(token);
        Ok(())
    }
    
    pub fn validate_file(&self, path: &Path) -> Result<FileValidationResult> {
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow!("Invalid file extension"))?
            .to_lowercase();
            
        if !self.allowed_extensions.contains(&extension) {
            return Ok(FileValidationResult {
                is_valid: false,
                file_type: extension,
                file_size: 0,
                validation_errors: vec!["Unsupported file type".to_string()],
                security_warnings: vec![],
                virus_scan_result: None,
            });
        }
        
        let metadata = std::fs::metadata(path)?;
        if metadata.len() > self.max_file_size {
            return Ok(FileValidationResult {
                is_valid: false,
                file_type: extension,
                file_size: metadata.len(),
                validation_errors: vec!["File too large".to_string()],
                security_warnings: vec![],
                virus_scan_result: None,
            });
        }
        
        let mut validation_errors = Vec::new();
        let mut security_warnings = Vec::new();
        
        // הפעלת כל הבדיקות המותקנות
        for validator in &self.file_validators {
            match validator.validate(path) {
                Ok(result) => {
                    validation_errors.extend(result.validation_errors);
                    security_warnings.extend(result.security_warnings);
                    
                    if let Some(virus_result) = result.virus_scan_result {
                        if !virus_result.is_clean {
                            return Ok(FileValidationResult {
                                is_valid: false,
                                file_type: extension,
                                file_size: metadata.len(),
                                validation_errors: vec!["Malware detected".to_string()],
                                security_warnings: virus_result.threats_found,
                                virus_scan_result: Some(virus_result),
                            });
                        }
                    }
                }
                Err(e) => {
                    security_warnings.push(format!("Validation error: {}", e));
                }
            }
        }
        
        Ok(FileValidationResult {
            is_valid: validation_errors.is_empty(),
            file_type: extension,
            file_size: metadata.len(),
            validation_errors,
            security_warnings,
            virus_scan_result: None,
        })
    }
    
    pub fn check_rate_limit(&self, ip: &str, limit: usize, window: Duration) -> Result<bool> {
        let mut rate_limits = self.rate_limits.lock().unwrap();
        let now = SystemTime::now();
        
        // ניקוי רשומות ישנות
        if let Some(timestamps) = rate_limits.get_mut(ip) {
            timestamps.retain(|&timestamp| {
                if let Ok(elapsed) = now.duration_since(timestamp) {
                    elapsed < window
                } else {
                    false
                }
            });
        }
        
        let timestamps = rate_limits.entry(ip.to_string()).or_insert_with(Vec::new);
        
        if timestamps.len() >= limit {
            // חסימת IP בעקבות חריגה ממגבלת הקצב
            self.blocked_ips.lock().unwrap().insert(
                ip.to_string(),
                now + Duration::from_secs(3600) // חסימה לשעה
            );
            
            return Ok(false);
        }
        
        timestamps.push(now);
        Ok(true)
    }
    
    pub fn is_ip_blocked(&self, ip: &str) -> bool {
        let blocked_ips = self.blocked_ips.lock().unwrap();
        
        if let Some(&block_until) = blocked_ips.get(ip) {
            if let Ok(elapsed) = SystemTime::now().duration_since(block_until) {
                elapsed.as_secs() == 0
            } else {
                true
            }
        } else {
            false
        }
    }
    
    pub fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm, Nonce,
        };
        
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|e| anyhow!("Encryption error: {}", e))?;
            
        let mut rng = rand::thread_rng();
        let mut nonce = [0u8; 12];
        rng.fill(&mut nonce);
        
        let encrypted = cipher
            .encrypt(Nonce::from_slice(&nonce), data)
            .map_err(|e| anyhow!("Encryption error: {}", e))?;
            
        let mut result = Vec::with_capacity(nonce.len() + encrypted.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&encrypted);
        
        Ok(result)
    }
    
    pub fn decrypt_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm, Nonce,
        };
        
        if encrypted_data.len() < 12 {
            return Err(anyhow!("Invalid encrypted data"));
        }
        
        let (nonce, ciphertext) = encrypted_data.split_at(12);
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|e| anyhow!("Decryption error: {}", e))?;
            
        cipher
            .decrypt(Nonce::from_slice(nonce), ciphertext)
            .map_err(|e| anyhow!("Decryption error: {}", e))
    }
    
    pub fn hash_password(&self, password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    pub fn verify_password(&self, password: &str, hash: &str) -> bool {
        let password_hash = self.hash_password(password);
        password_hash == hash
    }
    
    pub fn add_file_validator(&mut self, validator: Box<dyn FileValidator + Send + Sync>) {
        self.file_validators.push(validator);
    }
} 