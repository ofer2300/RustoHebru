use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokenizer {
    vocab: HashMap<String, i64>,
    reverse_vocab: HashMap<i64, String>,
    special_tokens: SpecialTokens,
    next_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialTokens {
    pub pad: String,
    pub bos: String,
    pub eos: String,
    pub unk: String,
}

impl Default for SpecialTokens {
    fn default() -> Self {
        Self {
            pad: "<PAD>".to_string(),
            bos: "<BOS>".to_string(),
            eos: "<EOS>".to_string(),
            unk: "<UNK>".to_string(),
        }
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        let mut tokenizer = Self {
            vocab: HashMap::new(),
            reverse_vocab: HashMap::new(),
            special_tokens: SpecialTokens::default(),
            next_id: 0,
        };
        
        // הוספת טוקנים מיוחדים
        tokenizer.add_token(&tokenizer.special_tokens.pad);
        tokenizer.add_token(&tokenizer.special_tokens.bos);
        tokenizer.add_token(&tokenizer.special_tokens.eos);
        tokenizer.add_token(&tokenizer.special_tokens.unk);
        
        tokenizer
    }
    
    pub fn add_token(&mut self, token: &str) -> i64 {
        if let Some(&id) = self.vocab.get(token) {
            id
        } else {
            let id = self.next_id;
            self.vocab.insert(token.to_string(), id);
            self.reverse_vocab.insert(id, token.to_string());
            self.next_id += 1;
            id
        }
    }
    
    pub fn tokenize(&self, text: &str) -> Vec<i64> {
        let mut tokens = Vec::new();
        
        // הוספת טוקן התחלה
        tokens.push(self.vocab[&self.special_tokens.bos]);
        
        // פיצול הטקסט למילים
        for word in text.split_whitespace() {
            // טיפול במילים מורכבות
            let subwords = self.split_to_subwords(word);
            
            for subword in subwords {
                let token_id = self.vocab
                    .get(&subword)
                    .copied()
                    .unwrap_or_else(|| self.vocab[&self.special_tokens.unk]);
                    
                tokens.push(token_id);
            }
        }
        
        // הוספת טוקן סיום
        tokens.push(self.vocab[&self.special_tokens.eos]);
        
        tokens
    }
    
    pub fn detokenize(&self, tokens: &[i64]) -> String {
        let mut text = String::new();
        let mut is_first = true;
        
        for &token_id in tokens {
            // דילוג על טוקנים מיוחדים
            if token_id == self.vocab[&self.special_tokens.bos] ||
               token_id == self.vocab[&self.special_tokens.eos] ||
               token_id == self.vocab[&self.special_tokens.pad] {
                continue;
            }
            
            let token = self.reverse_vocab
                .get(&token_id)
                .unwrap_or(&self.special_tokens.unk);
                
            // הוספת רווח בין מילים
            if !is_first && !token.starts_with("##") {
                text.push(' ');
            }
            
            // הסרת סימון תת-מילה
            let clean_token = token.trim_start_matches("##");
            text.push_str(clean_token);
            
            is_first = false;
        }
        
        text
    }
    
    fn split_to_subwords(&self, word: &str) -> Vec<String> {
        let mut subwords = Vec::new();
        let mut start = 0;
        
        while start < word.len() {
            let mut end = word.len();
            let mut found = false;
            
            // חיפוש תת-מילה הארוכה ביותר שקיימת באוצר המילים
            while end > start && !found {
                let subword = if start == 0 {
                    word[start..end].to_string()
                } else {
                    format!("##{}", &word[start..end])
                };
                
                if self.vocab.contains_key(&subword) {
                    subwords.push(subword);
                    found = true;
                } else {
                    end -= 1;
                }
            }
            
            // אם לא נמצאה תת-מילה, נוסיף תו בודד
            if !found {
                let subword = if start == 0 {
                    word[start..start+1].to_string()
                } else {
                    format!("##{}", &word[start..start+1])
                };
                subwords.push(subword);
                end = start + 1;
            }
            
            start = end;
        }
        
        subwords
    }
    
    pub fn train<I>(&mut self, texts: I, min_freq: usize)
    where
        I: IntoIterator<Item = String>
    {
        let mut token_counts = HashMap::new();
        
        // ספירת תדירויות של תת-מילים
        for text in texts {
            for word in text.split_whitespace() {
                let subwords = self.split_to_subwords(word);
                for subword in subwords {
                    *token_counts.entry(subword).or_insert(0) += 1;
                }
            }
        }
        
        // הוספת תת-מילים שמופיעות מספיק פעמים
        for (token, count) in token_counts {
            if count >= min_freq {
                self.add_token(&token);
            }
        }
    }
    
    pub fn save(&self, path: &str) -> Result<(), TokenizerError> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
    
    pub fn load(path: &str) -> Result<Self, TokenizerError> {
        let file = std::fs::File::open(path)?;
        let tokenizer = serde_json::from_reader(file)?;
        Ok(tokenizer)
    }
}

#[derive(Debug)]
pub enum TokenizerError {
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
}

impl From<std::io::Error> for TokenizerError {
    fn from(error: std::io::Error) -> Self {
        TokenizerError::IoError(error)
    }
}

impl From<serde_json::Error> for TokenizerError {
    fn from(error: serde_json::Error) -> Self {
        TokenizerError::SerializationError(error)
    }
}

impl std::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizerError::IoError(e) => write!(f, "שגיאת קלט/פלט: {}", e),
            TokenizerError::SerializationError(e) => write!(f, "שגיאת סריאליזציה: {}", e),
        }
    }
}

impl std::error::Error for TokenizerError {} 