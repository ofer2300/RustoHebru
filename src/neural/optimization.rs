use tch::{Tensor, nn, Device};
use std::sync::Arc;

/// הגדרות אופטימיזציה מתקדמות
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub learning_rate: f64,
    pub max_grad_norm: f64,
    pub weight_decay: f64,
    pub beta1: f64,
    pub beta2: f64,
    pub eps: f64,
    pub warmup_steps: i64,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.001,
            max_grad_norm: 1.0,
            weight_decay: 0.01,
            beta1: 0.9,
            beta2: 0.999,
            eps: 1e-8,
            warmup_steps: 4000,
        }
    }
}

/// אופטימייזר משופר עם תמיכה ב-gradient clipping ו-warmup
pub struct EnhancedOptimizer {
    optimizer: nn::Adam,
    config: OptimizationConfig,
    step: i64,
    device: Device,
}

impl EnhancedOptimizer {
    pub fn new(vs: &nn::VarStore, config: OptimizationConfig) -> Self {
        let mut adam_config = nn::AdamConfig::default();
        adam_config.beta1 = config.beta1;
        adam_config.beta2 = config.beta2;
        adam_config.eps = config.eps;
        adam_config.weight_decay = config.weight_decay;
        
        let optimizer = nn::Adam::new(vs.trainable_variables(), adam_config);
        
        Self {
            optimizer,
            config,
            step: 0,
            device: vs.device(),
        }
    }

    pub fn backward_step(&mut self, loss: &Tensor) {
        // חישוב גרדיאנטים
        loss.backward();
        
        // חישוב learning rate דינמי
        let lr = self.get_learning_rate();
        
        // Gradient Clipping
        self.clip_gradients();
        
        // עדכון משקולות
        self.optimizer.set_lr(lr);
        self.optimizer.step();
        
        // איפוס גרדיאנטים
        self.optimizer.zero_grad();
        
        self.step += 1;
    }

    fn get_learning_rate(&self) -> f64 {
        let step = self.step as f64;
        let warmup = self.config.warmup_steps as f64;
        
        if step < warmup {
            // Linear warmup
            self.config.learning_rate * (step / warmup)
        } else {
            // Cosine decay
            let progress = (step - warmup) / (100000.0 - warmup);
            let cosine_decay = 0.5 * (1.0 + (std::f64::consts::PI * progress).cos());
            self.config.learning_rate * cosine_decay
        }
    }

    fn clip_gradients(&self) {
        let mut total_norm = Tensor::zeros(&[], (tch::Kind::Float, self.device));
        
        // חישוב נורמה כוללת של הגרדיאנטים
        for param in self.optimizer.trainable_variables() {
            if let Some(grad) = param.grad() {
                let norm = grad.norm().pow_tensor_scalar(2.0);
                total_norm += norm;
            }
        }
        
        total_norm = total_norm.sqrt();
        let clip_coef = (self.config.max_grad_norm / (total_norm + 1e-6)).clamp(0.0, 1.0);
        
        // קיטום הגרדיאנטים
        for param in self.optimizer.trainable_variables() {
            if let Some(mut grad) = param.grad() {
                grad *= clip_coef;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learning_rate_schedule() {
        let vs = nn::VarStore::new(Device::Cpu);
        let config = OptimizationConfig::default();
        let mut optimizer = EnhancedOptimizer::new(&vs, config.clone());
        
        // בדיקת warmup
        optimizer.step = 0;
        assert_eq!(optimizer.get_learning_rate(), 0.0);
        
        optimizer.step = config.warmup_steps / 2;
        assert!(optimizer.get_learning_rate() < config.learning_rate);
        
        optimizer.step = config.warmup_steps;
        assert_eq!(optimizer.get_learning_rate(), config.learning_rate);
    }

    #[test]
    fn test_gradient_clipping() {
        let vs = nn::VarStore::new(Device::Cpu);
        let mut config = OptimizationConfig::default();
        config.max_grad_norm = 1.0;
        
        let optimizer = EnhancedOptimizer::new(&vs, config);
        
        let param = Tensor::rand(&[10], (tch::Kind::Float, Device::Cpu));
        let grad = Tensor::rand(&[10], (tch::Kind::Float, Device::Cpu)) * 10.0; // גרדיאנט גדול
        
        param.set_grad(Some(grad));
        optimizer.clip_gradients();
        
        let clipped_grad = param.grad().unwrap();
        assert!(clipped_grad.norm().double_value(&[]) <= 1.0);
    }
} 