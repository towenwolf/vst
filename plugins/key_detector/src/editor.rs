use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, Align, Color32, FontId, Layout, RichText, Vec2};
use nih_plug_egui::{create_egui_editor, EguiState};
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::{AnalysisOutput, KeyDetectorParams, Mode, NoteName};

const WINDOW_WIDTH: u32 = 300;
const WINDOW_HEIGHT: u32 = 180;

pub fn default_state() -> Arc<EguiState> {
    EguiState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT)
}

pub fn create(
    _params: Arc<KeyDetectorParams>,
    output: Arc<AnalysisOutput>,
    editor_state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        editor_state,
        (),
        |_, _| {},
        move |egui_ctx, _setter, _state| {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.add_space(10.0);

                    // Title
                    ui.label(
                        RichText::new("Key Detector")
                            .font(FontId::proportional(16.0))
                            .color(Color32::GRAY),
                    );

                    ui.add_space(20.0);

                    // Read current values
                    let root = output.root.load(Ordering::Relaxed) as usize;
                    let mode_val = output.mode.load(Ordering::Relaxed);
                    let confidence_raw = output.confidence.load(Ordering::Relaxed);
                    let confidence = confidence_raw as f32 / 100.0;

                    let note_name = NoteName::from(root);
                    let mode = if mode_val == 0 {
                        Mode::Major
                    } else {
                        Mode::Minor
                    };

                    // Format the key string
                    let note_str = match note_name {
                        NoteName::C => "C",
                        NoteName::CSharp => "C#",
                        NoteName::D => "D",
                        NoteName::DSharp => "D#",
                        NoteName::E => "E",
                        NoteName::F => "F",
                        NoteName::FSharp => "F#",
                        NoteName::G => "G",
                        NoteName::GSharp => "G#",
                        NoteName::A => "A",
                        NoteName::ASharp => "A#",
                        NoteName::B => "B",
                    };

                    let mode_str = match mode {
                        Mode::Major => "Major",
                        Mode::Minor => "Minor",
                    };

                    let key_string = format!("{} {}", note_str, mode_str);

                    // Color based on confidence
                    let key_color = if confidence >= 70.0 {
                        Color32::from_rgb(100, 200, 100) // Green for high confidence
                    } else if confidence >= 40.0 {
                        Color32::from_rgb(200, 200, 100) // Yellow for medium
                    } else {
                        Color32::from_rgb(150, 150, 150) // Gray for low
                    };

                    // Display the detected key (large)
                    ui.label(
                        RichText::new(&key_string)
                            .font(FontId::proportional(48.0))
                            .color(key_color),
                    );

                    ui.add_space(15.0);

                    // Display confidence
                    let confidence_text = format!("Confidence: {:.1}%", confidence);
                    ui.label(
                        RichText::new(&confidence_text)
                            .font(FontId::proportional(18.0))
                            .color(Color32::LIGHT_GRAY),
                    );

                    ui.add_space(10.0);

                    // Simple confidence bar
                    let bar_width = 200.0;
                    let bar_height = 8.0;
                    let (rect, _) =
                        ui.allocate_exact_size(Vec2::new(bar_width, bar_height), egui::Sense::hover());

                    let painter = ui.painter();

                    // Background
                    painter.rect_filled(rect, 4.0, Color32::from_rgb(40, 40, 40));

                    // Filled portion
                    let fill_width = (confidence / 100.0).clamp(0.0, 1.0) * bar_width;
                    let fill_rect = egui::Rect::from_min_size(rect.min, Vec2::new(fill_width, bar_height));
                    painter.rect_filled(fill_rect, 4.0, key_color);
                });
            });
        },
    )
}
