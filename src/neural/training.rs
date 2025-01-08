use tch::{nn, Device, Tensor, Kind};
use serde::{Serialize, Deserialize};
use std::time::Instant;
use crate::neural::model::{EnhancedTransformer, TranslationError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub batch_size: usize,
    pub num_epochs: usize,
    pub learning_rate: f64,
    pub min_freq: usize,
    pub max_seq_length: usize,
    pub validation_split: f64,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            batch_size: 32,
            num_epochs: 10,
            learning_rate: 0.001,
            min_freq: 5,
            max_seq_length: 128,
            validation_split: 0.1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMetrics {
    pub epoch: usize,
    pub train_loss: f64,
    pub train_accuracy: f64,
    pub val_loss: f64,
    pub val_accuracy: f64,
    pub epoch_time: f64,
}

pub struct Trainer {
    model: EnhancedTransformer,
    optimizer: nn::Optimizer,
    config: TrainingConfig,
    device: Device,
}

impl Trainer {
    pub fn new(model: EnhancedTransformer, config: TrainingConfig) -> Self {
        let device = Device::cuda_if_available();
        let optimizer = nn::Adam::default().build(&model.trainable_variables(), config.learning_rate)
            .expect("Failed to create optimizer");
            
        Self {
            model,
            optimizer,
            config,
            device,
        }
    }
    
    pub fn train<I>(&mut self, train_texts: I) -> Result<Vec<TrainingMetrics>, TranslationError>
    where
        I: IntoIterator<Item = (String, String)>
    {
        let start_time = Instant::now();
        let mut metrics = Vec::new();
        
        // הכנת הנתונים לאימון
        let (train_data, val_data) = self.prepare_data(train_texts)?;
        
        // אימון הטוקניזר
        self.model.train(
            train_data.iter().map(|(src, _)| src.clone())
                .chain(train_data.iter().map(|(_, tgt)| tgt.clone())),
            self.config.min_freq
        );
        
        // אימון המודל
        for epoch in 0..self.config.num_epochs {
            let epoch_start = Instant::now();
            
            // אימון על סט האימון
            let train_metrics = self.train_epoch(&train_data)?;
            
            // בדיקה על סט האימות
            let val_metrics = self.validate(&val_data)?;
            
            let epoch_time = epoch_start.elapsed().as_secs_f64();
            
            let epoch_metrics = TrainingMetrics {
                epoch,
                train_loss: train_metrics.0,
                train_accuracy: train_metrics.1,
                val_loss: val_metrics.0,
                val_accuracy: val_metrics.1,
                epoch_time,
            };
            
            metrics.push(epoch_metrics.clone());
            
            println!(
                "Epoch {}/{}: Train Loss = {:.4}, Train Acc = {:.4}, Val Loss = {:.4}, Val Acc = {:.4}, Time = {:.2}s",
                epoch + 1,
                self.config.num_epochs,
                epoch_metrics.train_loss,
                epoch_metrics.train_accuracy,
                epoch_metrics.val_loss,
                epoch_metrics.val_accuracy,
                epoch_metrics.epoch_time
            );
        }
        
        let total_time = start_time.elapsed().as_secs_f64();
        println!("Total training time: {:.2}s", total_time);
        
        Ok(metrics)
    }
    
    fn prepare_data<I>(&self, texts: I) -> Result<(Vec<(String, String)>, Vec<(String, String)>), TranslationError>
    where
        I: IntoIterator<Item = (String, String)>
    {
        let mut all_data: Vec<_> = texts.into_iter().collect();
        
        // ערבוב הנתונים
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        all_data.shuffle(&mut rng);
        
        // חלוקה לאימון ואימות
        let val_size = (all_data.len() as f64 * self.config.validation_split) as usize;
        let val_data = all_data.split_off(all_data.len() - val_size);
        
        Ok((all_data, val_data))
    }
    
    fn train_epoch(&mut self, train_data: &[(String, String)]) -> Result<(f64, f64), TranslationError> {
        let mut total_loss = 0.0;
        let mut total_correct = 0;
        let mut total_tokens = 0;
        
        // יצירת אצוות
        for batch in train_data.chunks(self.config.batch_size) {
            // הכנת הנתונים
            let (src_tokens, tgt_tokens): (Vec<_>, Vec<_>) = batch.iter()
                .map(|(src, tgt)| {
                    (
                        self.model.tokenizer().tokenize(src),
                        self.model.tokenizer().tokenize(tgt)
                    )
                })
                .unzip();
            
            // המרה לטנסורים
            let src_tensor = Tensor::stack(
                &src_tokens.iter()
                    .map(|tokens| Tensor::of_slice(tokens).to_device(self.device))
                    .collect::<Vec<_>>(),
                0
            );
            
            let tgt_tensor = Tensor::stack(
                &tgt_tokens.iter()
                    .map(|tokens| Tensor::of_slice(tokens).to_device(self.device))
                    .collect::<Vec<_>>(),
                0
            );
            
            // Forward pass
            let output = self.model.forward(&src_tensor)?;
            
            // חישוב Loss
            let loss = output.cross_entropy_loss(&tgt_tensor, None, Kind::Mean);
            
            // Backward pass
            self.optimizer.zero_grad();
            loss.backward();
            self.optimizer.step();
            
            // חישוב מדדים
            total_loss += loss.double_value(&[]);
            
            let predictions = output.argmax(-1, false);
            let correct = predictions.eq(&tgt_tensor).to_kind(Kind::Float).sum(Kind::Float).double_value(&[]);
            
            total_correct += correct as i64;
            total_tokens += tgt_tensor.size()[0] * tgt_tensor.size()[1];
        }
        
        let avg_loss = total_loss / train_data.len() as f64;
        let accuracy = total_correct as f64 / total_tokens as f64;
        
        Ok((avg_loss, accuracy))
    }
    
    fn validate(&self, val_data: &[(String, String)]) -> Result<(f64, f64), TranslationError> {
        let mut total_loss = 0.0;
        let mut total_correct = 0;
        let mut total_tokens = 0;
        
        for batch in val_data.chunks(self.config.batch_size) {
            let (src_tokens, tgt_tokens): (Vec<_>, Vec<_>) = batch.iter()
                .map(|(src, tgt)| {
                    (
                        self.model.tokenizer().tokenize(src),
                        self.model.tokenizer().tokenize(tgt)
                    )
                })
                .unzip();
            
            let src_tensor = Tensor::stack(
                &src_tokens.iter()
                    .map(|tokens| Tensor::of_slice(tokens).to_device(self.device))
                    .collect::<Vec<_>>(),
                0
            );
            
            let tgt_tensor = Tensor::stack(
                &tgt_tokens.iter()
                    .map(|tokens| Tensor::of_slice(tokens).to_device(self.device))
                    .collect::<Vec<_>>(),
                0
            );
            
            let output = self.model.forward(&src_tensor)?;
            
            let loss = output.cross_entropy_loss(&tgt_tensor, None, Kind::Mean);
            total_loss += loss.double_value(&[]);
            
            let predictions = output.argmax(-1, false);
            let correct = predictions.eq(&tgt_tensor).to_kind(Kind::Float).sum(Kind::Float).double_value(&[]);
            
            total_correct += correct as i64;
            total_tokens += tgt_tensor.size()[0] * tgt_tensor.size()[1];
        }
        
        let avg_loss = total_loss / val_data.len() as f64;
        let accuracy = total_correct as f64 / total_tokens as f64;
        
        Ok((avg_loss, accuracy))
    }
    
    pub fn save_checkpoint(&self, path: &str) -> Result<(), TranslationError> {
        self.model.save_model(path)?;
        
        let checkpoint = TrainingCheckpoint {
            config: self.config.clone(),
            optimizer_state: self.optimizer.serialize(),
        };
        
        let checkpoint_path = format!("{}_checkpoint.json", path);
        std::fs::write(
            &checkpoint_path,
            serde_json::to_string_pretty(&checkpoint).map_err(|e| TranslationError::ModelError(e.to_string()))?
        ).map_err(|e| TranslationError::ModelError(e.to_string()))?;
        
        Ok(())
    }
    
    pub fn load_checkpoint(path: &str) -> Result<Self, TranslationError> {
        let model = EnhancedTransformer::load_model(path)?;
        
        let checkpoint_path = format!("{}_checkpoint.json", path);
        let checkpoint: TrainingCheckpoint = serde_json::from_str(
            &std::fs::read_to_string(&checkpoint_path)
                .map_err(|e| TranslationError::ModelError(e.to_string()))?
        ).map_err(|e| TranslationError::ModelError(e.to_string()))?;
        
        let device = Device::cuda_if_available();
        let mut optimizer = nn::Adam::default()
            .build(&model.trainable_variables(), checkpoint.config.learning_rate)
            .expect("Failed to create optimizer");
            
        optimizer.deserialize(&checkpoint.optimizer_state);
        
        Ok(Self {
            model,
            optimizer,
            config: checkpoint.config,
            device,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct TrainingCheckpoint {
    config: TrainingConfig,
    optimizer_state: Vec<u8>,
} 