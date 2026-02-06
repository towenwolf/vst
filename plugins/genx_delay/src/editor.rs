//! Basic GUI editor for GenX Delay

use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, Color32, RichText, Slider};
use nih_plug_egui::{create_egui_editor, EguiState};
use std::sync::Arc;

use crate::{DelayMode, GenXDelayParams};

const WINDOW_WIDTH: u32 = 400;
const WINDOW_HEIGHT: u32 = 600;

pub fn default_state() -> Arc<EguiState> {
    EguiState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT)
}

pub fn create(
    params: Arc<GenXDelayParams>,
    editor_state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        editor_state,
        (),
        |_, _| {},
        move |egui_ctx, setter, _state| {
            // Dark theme
            egui_ctx.set_visuals(egui::Visuals::dark());

            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new("GenX Delay")
                            .size(24.0)
                            .color(Color32::from_rgb(200, 160, 100)),
                    );
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(10.0);
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    // === TIME SECTION ===
                    ui.group(|ui| {
                        ui.label(RichText::new("TIME").strong());
                        ui.add_space(5.0);

                        // Delay Time
                        let mut delay_time = params.delay_time.value();
                        ui.horizontal(|ui| {
                            ui.label("Delay Time:");
                            if ui
                                .add(Slider::new(&mut delay_time, 1.0..=2500.0).suffix(" ms"))
                                .changed()
                            {
                                setter.begin_set_parameter(&params.delay_time);
                                setter.set_parameter(&params.delay_time, delay_time);
                                setter.end_set_parameter(&params.delay_time);
                            }
                        });

                        // Tempo Sync
                        let mut tempo_sync = params.tempo_sync.value();
                        if ui.checkbox(&mut tempo_sync, "Tempo Sync").changed() {
                            setter.begin_set_parameter(&params.tempo_sync);
                            setter.set_parameter(&params.tempo_sync, tempo_sync);
                            setter.end_set_parameter(&params.tempo_sync);
                        }

                        // Note Division (only shown when tempo sync is on)
                        if tempo_sync {
                            let current_div = params.note_division.value();
                            let div_names = [
                                "1/1", "1/2", "1/2 dot", "1/2 tri",
                                "1/4", "1/4 dot", "1/4 tri",
                                "1/8", "1/8 dot", "1/8 tri",
                                "1/16", "1/16 dot", "1/16 tri",
                            ];
                            let mut selected = current_div as usize;
                            ui.horizontal(|ui| {
                                ui.label("Division:");
                                egui::ComboBox::from_id_salt("note_div")
                                    .selected_text(div_names[selected])
                                    .show_ui(ui, |ui| {
                                        for (i, name) in div_names.iter().enumerate() {
                                            if ui.selectable_value(&mut selected, i, *name).changed() {
                                                setter.begin_set_parameter(&params.note_division);
                                                setter.set_parameter_normalized(
                                                    &params.note_division,
                                                    i as f32 / (div_names.len() - 1) as f32,
                                                );
                                                setter.end_set_parameter(&params.note_division);
                                            }
                                        }
                                    });
                            });
                        }
                    });

                    ui.add_space(10.0);

                    // === MAIN SECTION ===
                    ui.group(|ui| {
                        ui.label(RichText::new("MAIN").strong());
                        ui.add_space(5.0);

                        // Feedback
                        let mut feedback = params.feedback.value();
                        ui.horizontal(|ui| {
                            ui.label("Feedback:");
                            if ui
                                .add(Slider::new(&mut feedback, 0.0..=0.95).show_value(true))
                                .changed()
                            {
                                setter.begin_set_parameter(&params.feedback);
                                setter.set_parameter(&params.feedback, feedback);
                                setter.end_set_parameter(&params.feedback);
                            }
                        });

                        // Mix
                        let mut mix = params.mix.value();
                        ui.horizontal(|ui| {
                            ui.label("Mix:");
                            if ui
                                .add(Slider::new(&mut mix, 0.0..=1.0).show_value(true))
                                .changed()
                            {
                                setter.begin_set_parameter(&params.mix);
                                setter.set_parameter(&params.mix, mix);
                                setter.end_set_parameter(&params.mix);
                            }
                        });

                        // Mode
                        ui.horizontal(|ui| {
                            ui.label("Mode:");
                            let current_mode = params.mode.value();
                            if ui
                                .selectable_label(current_mode == DelayMode::Digital, "Digital")
                                .clicked()
                            {
                                setter.begin_set_parameter(&params.mode);
                                setter.set_parameter(&params.mode, DelayMode::Digital);
                                setter.end_set_parameter(&params.mode);
                            }
                            if ui
                                .selectable_label(current_mode == DelayMode::Analog, "Analog")
                                .clicked()
                            {
                                setter.begin_set_parameter(&params.mode);
                                setter.set_parameter(&params.mode, DelayMode::Analog);
                                setter.end_set_parameter(&params.mode);
                            }
                        });
                    });

                    ui.add_space(10.0);

                    // === STEREO SECTION ===
                    ui.group(|ui| {
                        ui.label(RichText::new("STEREO").strong());
                        ui.add_space(5.0);

                        // Ping Pong
                        let mut ping_pong = params.ping_pong.value();
                        if ui.checkbox(&mut ping_pong, "Ping Pong").changed() {
                            setter.begin_set_parameter(&params.ping_pong);
                            setter.set_parameter(&params.ping_pong, ping_pong);
                            setter.end_set_parameter(&params.ping_pong);
                        }

                        // Stereo Offset
                        let mut offset = params.stereo_offset.value();
                        ui.horizontal(|ui| {
                            ui.label("Offset:");
                            if ui
                                .add(Slider::new(&mut offset, 0.0..=50.0).suffix(" ms"))
                                .changed()
                            {
                                setter.begin_set_parameter(&params.stereo_offset);
                                setter.set_parameter(&params.stereo_offset, offset);
                                setter.end_set_parameter(&params.stereo_offset);
                            }
                        });
                    });

                    ui.add_space(10.0);

                    // === TONE SECTION ===
                    ui.group(|ui| {
                        ui.label(RichText::new("TONE").strong());
                        ui.add_space(5.0);

                        // High-Pass
                        let mut hp = params.highpass_freq.value();
                        ui.horizontal(|ui| {
                            ui.label("High-Pass:");
                            if ui
                                .add(Slider::new(&mut hp, 20.0..=1000.0).suffix(" Hz").logarithmic(true))
                                .changed()
                            {
                                setter.begin_set_parameter(&params.highpass_freq);
                                setter.set_parameter(&params.highpass_freq, hp);
                                setter.end_set_parameter(&params.highpass_freq);
                            }
                        });

                        // Low-Pass
                        let mut lp = params.lowpass_freq.value();
                        ui.horizontal(|ui| {
                            ui.label("Low-Pass:");
                            if ui
                                .add(Slider::new(&mut lp, 500.0..=20000.0).suffix(" Hz").logarithmic(true))
                                .changed()
                            {
                                setter.begin_set_parameter(&params.lowpass_freq);
                                setter.set_parameter(&params.lowpass_freq, lp);
                                setter.end_set_parameter(&params.lowpass_freq);
                            }
                        });
                    });

                    ui.add_space(10.0);

                    // === MODULATION SECTION (Analog only) ===
                    let is_analog = params.mode.value() == DelayMode::Analog;
                    ui.add_enabled_ui(is_analog, |ui| {
                        ui.group(|ui| {
                            let label = if is_analog {
                                RichText::new("MODULATION").strong()
                            } else {
                                RichText::new("MODULATION (Analog only)").weak()
                            };
                            ui.label(label);
                            ui.add_space(5.0);

                            // Rate
                            let mut rate = params.mod_rate.value();
                            ui.horizontal(|ui| {
                                ui.label("Rate:");
                                if ui
                                    .add(Slider::new(&mut rate, 0.1..=5.0).suffix(" Hz"))
                                    .changed()
                                {
                                    setter.begin_set_parameter(&params.mod_rate);
                                    setter.set_parameter(&params.mod_rate, rate);
                                    setter.end_set_parameter(&params.mod_rate);
                                }
                            });

                            // Depth
                            let mut depth = params.mod_depth.value();
                            ui.horizontal(|ui| {
                                ui.label("Depth:");
                                if ui.add(Slider::new(&mut depth, 0.0..=1.0)).changed() {
                                    setter.begin_set_parameter(&params.mod_depth);
                                    setter.set_parameter(&params.mod_depth, depth);
                                    setter.end_set_parameter(&params.mod_depth);
                                }
                            });

                            // Drive
                            let mut drive = params.drive.value();
                            ui.horizontal(|ui| {
                                ui.label("Drive:");
                                if ui.add(Slider::new(&mut drive, 0.0..=1.0)).changed() {
                                    setter.begin_set_parameter(&params.drive);
                                    setter.set_parameter(&params.drive, drive);
                                    setter.end_set_parameter(&params.drive);
                                }
                            });
                        });
                    });

                    ui.add_space(10.0);

                    // === DUCKING SECTION ===
                    ui.group(|ui| {
                        ui.label(RichText::new("DUCK").strong());
                        ui.add_space(5.0);

                        // Amount
                        let mut amount = params.duck_amount.value();
                        ui.horizontal(|ui| {
                            ui.label("Amount:");
                            if ui.add(Slider::new(&mut amount, 0.0..=1.0)).changed() {
                                setter.begin_set_parameter(&params.duck_amount);
                                setter.set_parameter(&params.duck_amount, amount);
                                setter.end_set_parameter(&params.duck_amount);
                            }
                        });

                        // Threshold
                        let mut threshold = params.duck_threshold.value();
                        ui.horizontal(|ui| {
                            ui.label("Threshold:");
                            if ui.add(Slider::new(&mut threshold, 0.0..=1.0)).changed() {
                                setter.begin_set_parameter(&params.duck_threshold);
                                setter.set_parameter(&params.duck_threshold, threshold);
                                setter.end_set_parameter(&params.duck_threshold);
                            }
                        });
                    });
                });
            });
        },
    )
}
