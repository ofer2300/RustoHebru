use tch::{nn, Device, Tensor};
use serde::{Serialize, Deserialize};
use crate::tokenizer::Tokenizer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationInput {
    pub text: String,
    pub source_lang: String, 
    pub target_lang: String,
    pub domain: Option<String>,
    pub style: Option<String>,
    pub formality: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationOutput {
    pub text: String,
    pub alternatives: Vec<Alternative>,
    pub confidence: f32,
    pub metadata: TranslationMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub text: String,
    pub confidence: f32,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationMetadata {
    pub source_lang: String,
    pub target_lang: String,
    pub domain: Option<String>,
    pub style: Option<String>,
    pub formality: Option<String>,
    pub model_version: String,
    pub processing_time: f32,
}

pub struct EnhancedTransformer {
    encoder: nn::Sequential,
    decoder: nn::Sequential,
    embedding: nn::Embedding,
    device: Device,
    vocab_size: i64,
    d_model: i64,
    tokenizer: Tokenizer,
}

impl EnhancedTransformer {
    pub fn new() -> Self {
        let device = Device::cuda_if_available();
        let vocab_size = 50000;
        let d_model = 512;
        let vs = nn::VarStore::new(device);
        let root = vs.root();
        
        let embedding = nn::embedding(&root, vocab_size, d_model, Default::default());
        
        let encoder = nn::seq()
            .add(Self::encoder_layer(&root, d_model))
            .add(Self::encoder_layer(&root, d_model))
            .add(Self::encoder_layer(&root, d_model));
            
        let decoder = nn::seq()
            .add(Self::decoder_layer(&root, d_model))
            .add(Self::decoder_layer(&root, d_model))
            .add(Self::decoder_layer(&root, d_model));
            
        Self {
            encoder,
            decoder, 
            embedding,
            device,
            vocab_size,
            d_model,
            tokenizer: Tokenizer::new(),
        }
    }
    
    fn encoder_layer(vs: &nn::Path, d_model: i64) -> nn::Sequential {
        let config = Default::default();
        nn::seq()
            .add(nn::linear(vs, d_model, d_model, config))
            .add_fn(|x| x.relu())
            .add(nn::dropout(0.1))
            .add(nn::layer_norm(vs, vec![d_model], Default::default()))
    }
    
    fn decoder_layer(vs: &nn::Path, d_model: i64) -> nn::Sequential {
        let config = Default::default();
        nn::seq()
            .add(nn::linear(vs, d_model, d_model, config))
            .add_fn(|x| x.relu())
            .add(nn::dropout(0.1))
            .add(nn::layer_norm(vs, vec![d_model], Default::default()))
    }
    
    pub async fn translate_text(&self, input: &TranslationInput) -> Result<TranslationOutput, TranslationError> {
        let start_time = std::time::Instant::now();
        
        // טוקניזציה של טקסט הקלט
        let tokens = self.tokenizer.tokenize(&input.text);
        
        // המרה לטנסור והעברה למכשיר המתאים
        let input_tensor = Tensor::of_slice(&tokens).to_device(self.device);
        
        // קידוד הקלט
        let encoded = self.encode(input_tensor)?;
        
        // פענוח ויצירת התרגום
        let output_tokens = self.decode(encoded)?;
        
        // דטוקניזציה והמרה חזרה לטקסט
        let translated_text = self.tokenizer.detokenize(&output_tokens);
        
        let processing_time = start_time.elapsed().as_secs_f32();
        
        Ok(TranslationOutput {
            text: translated_text,
            alternatives: vec![],
            confidence: 0.95, // יש להחליף בחישוב אמיתי
            metadata: TranslationMetadata {
                source_lang: input.source_lang.clone(),
                target_lang: input.target_lang.clone(),
                domain: input.domain.clone(),
                style: input.style.clone(),
                formality: input.formality.clone(),
                model_version: "1.0.0".to_string(),
                processing_time,
            },
        })
    }
    
    fn encode(&self, input: Tensor) -> Result<Tensor, TranslationError> {
        let embedded = self.embedding.forward(&input);
        Ok(self.encoder.forward(&embedded))
    }
    
    fn decode(&self, encoded: Tensor) -> Result<Vec<i64>, TranslationError> {
        let decoded = self.decoder.forward(&encoded);
        Ok(decoded.size1()?.iter()
            .map(|x| x as i64)
            .collect())
    }
    
    pub fn train<I>(&mut self, texts: I, min_freq: usize)
    where
        I: IntoIterator<Item = String>
    {
        // אימון הטוקניזר
        self.tokenizer.train(texts, min_freq);
        
        // עדכון גודל אוצר המילים במודל
        self.vocab_size = self.tokenizer.vocab_size() as i64;
        
        // יצירת אמבדינג חדש עם הגודל המעודכן
        let vs = nn::VarStore::new(self.device);
        let root = vs.root();
        self.embedding = nn::embedding(&root, self.vocab_size, self.d_model, Default::default());
    }
    
    pub fn save_model(&self, path: &str) -> Result<(), TranslationError> {
        // שמירת המודל
        let vs = nn::VarStore::new(self.device);
        vs.save(path).map_err(|e| TranslationError::ModelError(e.to_string()))?;
        
        // שמירת הטוקניזר
        let tokenizer_path = format!("{}_tokenizer.json", path);
        self.tokenizer.save(&tokenizer_path)
            .map_err(|e| TranslationError::ModelError(e.to_string()))?;
            
        Ok(())
    }
    
    pub fn load_model(path: &str) -> Result<Self, TranslationError> {
        let device = Device::cuda_if_available();
        
        // טעינת המודל
        let vs = nn::VarStore::new(device);
        vs.load(path).map_err(|e| TranslationError::ModelError(e.to_string()))?;
        
        // טעינת הטוקניזר
        let tokenizer_path = format!("{}_tokenizer.json", path);
        let tokenizer = Tokenizer::load(&tokenizer_path)
            .map_err(|e| TranslationError::ModelError(e.to_string()))?;
            
        let vocab_size = tokenizer.vocab_size() as i64;
        let d_model = 512;
        let root = vs.root();
        
        let embedding = nn::embedding(&root, vocab_size, d_model, Default::default());
        
        let encoder = nn::seq()
            .add(Self::encoder_layer(&root, d_model))
            .add(Self::encoder_layer(&root, d_model))
            .add(Self::encoder_layer(&root, d_model));
            
        let decoder = nn::seq()
            .add(Self::decoder_layer(&root, d_model))
            .add(Self::decoder_layer(&root, d_model))
            .add(Self::decoder_layer(&root, d_model));
            
        Ok(Self {
            encoder,
            decoder,
            embedding,
            device,
            vocab_size,
            d_model,
            tokenizer,
        })
    }
}

#[derive(Debug)]
pub enum TranslationError {
    EncodingError(String),
    DecodingError(String),
    ModelError(String),
}

impl std::fmt::Display for TranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncodingError(msg) => write!(f, "שגיאת קידוד: {}", msg),
            Self::DecodingError(msg) => write!(f, "שגיאת פענוח: {}", msg),
            Self::ModelError(msg) => write!(f, "שגיאת מודל: {}", msg),
        }
    }
}

impl std::error::Error for TranslationError {} 