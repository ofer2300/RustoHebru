use eframe::egui;
use tokio;
use crate::quality_control::{QualityControl, IssueSeverity};
use crate::translation_engine::TranslationEngine;
use std::sync::Arc;

pub struct ModernGui {
    input_text: String,
    output_text: String,
    quality_control: QualityControl,
    translation_engine: Arc<TranslationEngine>,
}

impl ModernGui {
    pub fn new(translation_engine: Arc<TranslationEngine>) -> Self {
        Self {
            input_text: String::new(),
            output_text: String::new(),
            quality_control: QualityControl::new(),
            translation_engine,
        }
    }
}

impl eframe::App for ModernGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("תרגום עברית-רוסית");
            
            ui.horizontal(|ui| {
                ui.label("טקסט מקור:");
                ui.text_edit_multiline(&mut self.input_text);
            });

            if ui.button("תרגם").clicked() {
                let text = self.input_text.clone();
                let engine = self.translation_engine.clone();
                
                tokio::spawn(async move {
                    if let Ok(translated) = engine.translate(&text, "he", "ru").await {
                        // TODO: עדכון התרגום בממשק
                        println!("תרגום: {}", translated);
                    }
                });
            }

            ui.horizontal(|ui| {
                ui.label("תרגום:");
                ui.text_edit_multiline(&mut self.output_text);
            });
        });
    }
} 