//! GUI editor for GenX Delay — Woodstock 99 theme

use nih_plug::prelude::*;
use nih_plug_egui::egui;
use nih_plug_egui::resizable_window::ResizableWindow;
use nih_plug_egui::{create_egui_editor, EguiState};
use std::sync::Arc;

use crate::GenXDelayParams;

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 420;

// Woodstock 99 color palette
const BG_MAIN: egui::Color32 = egui::Color32::from_rgb(235, 228, 215);
const BG_PANEL: egui::Color32 = egui::Color32::from_rgb(215, 205, 190);
const TEXT_DARK: egui::Color32 = egui::Color32::from_rgb(45, 40, 35);
const TRIBAL_BROWN: egui::Color32 = egui::Color32::from_rgb(75, 55, 40);
const ACCENT_WARM: egui::Color32 = egui::Color32::from_rgb(180, 95, 65);
const ACCENT_OLIVE: egui::Color32 = egui::Color32::from_rgb(105, 115, 80);
const ACCENT_NAVY: egui::Color32 = egui::Color32::from_rgb(60, 70, 90);
const RUST: egui::Color32 = egui::Color32::from_rgb(140, 75, 50);
const DOVE_GOLD: egui::Color32 = egui::Color32::from_rgb(175, 155, 100);

pub fn default_state() -> Arc<EguiState> {
    EguiState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT)
}

fn apply_theme(ctx: &egui::Context, scale: f32) {
    let mut visuals = egui::Visuals::light();

    // Background
    visuals.panel_fill = BG_MAIN;
    visuals.window_fill = BG_MAIN;
    visuals.extreme_bg_color = BG_PANEL;

    // Widget colors (stroke widths scale)
    visuals.widgets.inactive.bg_fill = BG_PANEL;
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0 * scale, TEXT_DARK);
    visuals.widgets.inactive.weak_bg_fill = BG_PANEL;

    visuals.widgets.hovered.bg_fill = ACCENT_OLIVE;
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5 * scale, BG_MAIN);
    visuals.widgets.hovered.weak_bg_fill = ACCENT_OLIVE;

    visuals.widgets.active.bg_fill = ACCENT_OLIVE;
    visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0 * scale, BG_MAIN);
    visuals.widgets.active.weak_bg_fill = ACCENT_OLIVE;

    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0 * scale, TEXT_DARK);
    visuals.widgets.noninteractive.bg_fill = BG_MAIN;

    // Slider rail
    visuals.selection.bg_fill = ACCENT_OLIVE;
    visuals.selection.stroke = egui::Stroke::new(1.0 * scale, ACCENT_OLIVE);

    // Window rounding and borders
    visuals.window_stroke = egui::Stroke::new(1.0 * scale, TRIBAL_BROWN);

    ctx.set_visuals(visuals);

    // Font sizes — scale proportionally with window
    let mut style = (*ctx.style()).clone();
    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::proportional(24.0 * scale));
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(11.0 * scale));
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(11.0 * scale));
    style
        .text_styles
        .insert(egui::TextStyle::Monospace, egui::FontId::monospace(11.0 * scale));
    style
        .text_styles
        .insert(egui::TextStyle::Small, egui::FontId::proportional(9.0 * scale));

    // Widget sizing — scale interaction targets and spacing
    style.spacing.interact_size = egui::vec2(40.0 * scale, 18.0 * scale);
    style.spacing.slider_width = 100.0 * scale;
    style.spacing.item_spacing = egui::vec2(8.0 * scale, 3.0 * scale);
    style.spacing.button_padding = egui::vec2(4.0 * scale, 1.0 * scale);

    ctx.set_style(style);
}

fn handle_bool_param(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    param: &BoolParam,
    value: &mut bool,
    label: &str,
) {
    if ui.checkbox(value, label).changed() {
        setter.begin_set_parameter(param);
        setter.set_parameter(param, *value);
        setter.end_set_parameter(param);
    }
}

fn handle_slider_param(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    param: &FloatParam,
    value: &mut f32,
    label: &str,
    range: std::ops::RangeInclusive<f32>,
) {
    let response = ui.add(egui::Slider::new(value, range).text(label));
    if response.drag_started() {
        setter.begin_set_parameter(param);
    }
    if response.changed() {
        setter.set_parameter(param, *value);
    }
    if response.drag_stopped() {
        setter.end_set_parameter(param);
    }
}

fn section_label(ui: &mut egui::Ui, text: &str, color: egui::Color32, scale: f32) {
    ui.label(egui::RichText::new(text).small().strong().color(color));
    ui.add_space(4.0 * scale);
}

/// Compute a uniform scale factor from the current window size relative to base.
fn content_scale(editor_state: &EguiState) -> f32 {
    let (w, h) = editor_state.size();
    let sx = w as f32 / WINDOW_WIDTH as f32;
    let sy = h as f32 / WINDOW_HEIGHT as f32;
    sx.min(sy).max(0.5) // clamp to prevent illegibly small text
}

pub fn create(
    params: Arc<GenXDelayParams>,
    editor_state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    let state_for_resize = editor_state.clone();
    create_egui_editor(
        editor_state,
        (),
        |_, _| {},
        move |egui_ctx, setter, _state| {
            let scale = content_scale(&state_for_resize);
            apply_theme(egui_ctx, scale);

            ResizableWindow::new("genx_delay")
                .min_size([600.0, 420.0])
                .show(egui_ctx, &state_for_resize, |ui| {
                    // ── Header ──
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("GENX DELAY").heading().color(TEXT_DARK));
                        ui.label(
                            egui::RichText::new("— WOODSTOCK 99 —")
                                .small()
                                .color(TRIBAL_BROWN),
                        );
                    });

                    ui.add_space(10.0 * scale);

                    // ── Row 1: TIME | MAIN | STEREO ──
                    ui.columns(3, |cols| {
                        // TIME section
                        cols[0].group(|ui| {
                            section_label(ui, "TIME", ACCENT_WARM, scale);

                            let mut delay_time_value = params.delay_time.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.delay_time,
                                &mut delay_time_value,
                                "Delay (ms)",
                                1.0..=2500.0,
                            );

                            ui.add_space(4.0 * scale);
                            let mut reverse_value = params.reverse.value();
                            handle_bool_param(
                                ui,
                                setter,
                                &params.reverse,
                                &mut reverse_value,
                                "Reverse",
                            );

                            // Tempo Sync + Note Division — controls wired in GDX-02
                            ui.add_space(4.0 * scale);
                            ui.label(
                                egui::RichText::new("Sync / Div")
                                    .small()
                                    .color(TEXT_DARK),
                            );
                        });

                        // MAIN section
                        cols[1].group(|ui| {
                            section_label(ui, "MAIN", ACCENT_OLIVE, scale);

                            let mut feedback_value = params.feedback.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.feedback,
                                &mut feedback_value,
                                "Feedback",
                                0.0..=0.95,
                            );

                            let mut mix_value = params.mix.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.mix,
                                &mut mix_value,
                                "Mix",
                                0.0..=1.0,
                            );

                            // Mode selector — wired in GDX-02
                            ui.add_space(4.0 * scale);
                            ui.label(
                                egui::RichText::new("Mode")
                                    .small()
                                    .color(TEXT_DARK),
                            );
                        });

                        // STEREO section
                        cols[2].group(|ui| {
                            section_label(ui, "STEREO", ACCENT_NAVY, scale);

                            // Ping Pong + Stereo Offset — controls wired in GDX-02
                            ui.label(
                                egui::RichText::new("Ping Pong")
                                    .small()
                                    .color(TEXT_DARK),
                            );
                            ui.add_space(4.0 * scale);
                            ui.label(
                                egui::RichText::new("Offset")
                                    .small()
                                    .color(TEXT_DARK),
                            );
                        });
                    });

                    ui.add_space(8.0 * scale);

                    // ── Row 2: TONE | MODULATION | DUCK ──
                    ui.columns(3, |cols| {
                        // TONE section
                        cols[0].group(|ui| {
                            section_label(ui, "TONE", TRIBAL_BROWN, scale);

                            // HP / LP — controls wired in GDX-02
                            ui.label(
                                egui::RichText::new("High-Pass")
                                    .small()
                                    .color(TEXT_DARK),
                            );
                            ui.add_space(4.0 * scale);
                            ui.label(
                                egui::RichText::new("Low-Pass")
                                    .small()
                                    .color(TEXT_DARK),
                            );
                        });

                        // MODULATION section
                        cols[1].group(|ui| {
                            section_label(ui, "MODULATION", RUST, scale);
                            ui.label(
                                egui::RichText::new("(Analog only)")
                                    .small()
                                    .color(RUST),
                            );

                            // Rate / Depth / Drive — controls wired in GDX-02
                            ui.add_space(4.0 * scale);
                            ui.label(
                                egui::RichText::new("Rate / Depth / Drive")
                                    .small()
                                    .color(TEXT_DARK),
                            );
                        });

                        // DUCK section
                        cols[2].group(|ui| {
                            section_label(ui, "DUCK", DOVE_GOLD, scale);

                            // Amount / Threshold — controls wired in GDX-02
                            ui.label(
                                egui::RichText::new("Amount")
                                    .small()
                                    .color(TEXT_DARK),
                            );
                            ui.add_space(4.0 * scale);
                            ui.label(
                                egui::RichText::new("Threshold")
                                    .small()
                                    .color(TEXT_DARK),
                            );
                        });
                    });
                });
        },
    )
}
