use tch::{Tensor, nn};
use std::borrow::Borrow;

/// שכבת נורמליזציה משופרת עם תמיכה ב-residual connections
pub struct EnhancedLayerNorm {
    layer_norm: nn::LayerNorm,
    dropout: f64,
    residual: bool,
}

impl EnhancedLayerNorm {
    pub fn new(vs: impl Borrow<nn::Path>, size: i64, dropout: f64, residual: bool) -> Self {
        let layer_norm = nn::layer_norm(vs, vec![size], Default::default());
        Self {
            layer_norm,
            dropout,
            residual,
        }
    }

    pub fn forward(&self, x: &Tensor, training: bool) -> Tensor {
        let mut out = self.layer_norm.forward(x);
        
        if training && self.dropout > 0.0 {
            out = out.dropout(self.dropout, training);
        }
        
        if self.residual {
            out = out + x;
        }
        
        out
    }
}

/// נורמליזציה מותאמת לתרגום
pub struct TranslationNorm {
    input_norm: EnhancedLayerNorm,
    output_norm: EnhancedLayerNorm,
    adaptive_factor: Tensor,
}

impl TranslationNorm {
    pub fn new(vs: impl Borrow<nn::Path>, size: i64, dropout: f64) -> Self {
        let vs = vs.borrow();
        
        Self {
            input_norm: EnhancedLayerNorm::new(&vs / "input_norm", size, dropout, true),
            output_norm: EnhancedLayerNorm::new(&vs / "output_norm", size, dropout, true),
            adaptive_factor: vs.var("adaptive_factor", &[1], Default::default()),
        }
    }

    pub fn forward_encoder(&self, x: &Tensor, training: bool) -> Tensor {
        let factor = self.adaptive_factor.sigmoid();
        let normalized = self.input_norm.forward(x, training);
        normalized * factor
    }

    pub fn forward_decoder(&self, x: &Tensor, training: bool) -> Tensor {
        let factor = self.adaptive_factor.sigmoid();
        let normalized = self.output_norm.forward(x, training);
        normalized * factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tch::Device;

    #[test]
    fn test_layer_norm() {
        let vs = nn::VarStore::new(Device::Cpu);
        let norm = EnhancedLayerNorm::new(&vs.root(), 10, 0.1, true);
        
        let x = Tensor::rand(&[5, 10], (tch::Kind::Float, Device::Cpu));
        let out = norm.forward(&x, true);
        
        assert_eq!(out.size(), x.size());
        
        // בדיקת נורמליזציה
        let mean = out.mean_dim(&[-1], true, tch::Kind::Float);
        let std = out.std_dim(&[-1], true, 0);
        
        assert!(mean.abs().max().double_value(&[]) < 1e-6);
        assert!((std - 1.0).abs().max().double_value(&[]) < 1e-6);
    }

    #[test]
    fn test_translation_norm() {
        let vs = nn::VarStore::new(Device::Cpu);
        let norm = TranslationNorm::new(&vs.root(), 10, 0.1);
        
        let x = Tensor::rand(&[5, 10], (tch::Kind::Float, Device::Cpu));
        
        let enc_out = norm.forward_encoder(&x, true);
        let dec_out = norm.forward_decoder(&x, true);
        
        assert_eq!(enc_out.size(), x.size());
        assert_eq!(dec_out.size(), x.size());
    }
} 