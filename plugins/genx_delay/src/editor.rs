//! GUI editor for GenX Delay — Woodstock 99 theme

use nih_plug::prelude::*;
use nih_plug_egui::egui;
use nih_plug_egui::resizable_window::ResizableWindow;
use nih_plug_egui::{create_egui_editor, EguiState};
use std::sync::Arc;

use crate::{DelayMode, GenXDelayParams, NoteDivision};

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

/// Horizontal button group for small enums (e.g. Mode: Digital | Analog).
fn handle_enum_buttons<T: Enum + PartialEq + Copy + 'static>(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    param: &EnumParam<T>,
    label: &str,
) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).small().color(TEXT_DARK));
        let current = param.value();
        for (idx, name) in T::variants().iter().enumerate() {
            let variant = T::from_index(idx);
            if ui.selectable_label(current == variant, *name).clicked() {
                setter.begin_set_parameter(param);
                setter.set_parameter(param, variant);
                setter.end_set_parameter(param);
            }
        }
    });
}

/// Dropdown combo box for large enums (e.g. Note Division with 13 options).
fn handle_enum_combobox<T: Enum + PartialEq + Copy + 'static>(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    param: &EnumParam<T>,
    label: &str,
    id_salt: &str,
) {
    let current = param.value();
    let current_name = T::variants()[current.to_index()];
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).small().color(TEXT_DARK));
        egui::ComboBox::from_id_salt(id_salt)
            .selected_text(current_name)
            .show_ui(ui, |ui| {
                for (idx, name) in T::variants().iter().enumerate() {
                    let variant = T::from_index(idx);
                    if ui.selectable_label(current == variant, *name).clicked() {
                        setter.begin_set_parameter(param);
                        setter.set_parameter(param, variant);
                        setter.end_set_parameter(param);
                    }
                }
            });
    });
}

fn section_label(ui: &mut egui::Ui, text: &str, color: egui::Color32, scale: f32) {
    ui.label(egui::RichText::new(text).small().strong().color(color));
    ui.add_space(4.0 * scale);
}

#[cfg(test)]
pub(crate) const GUI_NOTE_DIVISION_OPTIONS: &[(NoteDivision, &str)] = &[
    (NoteDivision::Whole, "1/1"),
    (NoteDivision::Half, "1/2"),
    (NoteDivision::HalfDotted, "1/2d"),
    (NoteDivision::HalfTriplet, "1/2t"),
    (NoteDivision::Quarter, "1/4"),
    (NoteDivision::QuarterDotted, "1/4d"),
    (NoteDivision::QuarterTriplet, "1/4t"),
    (NoteDivision::Eighth, "1/8"),
    (NoteDivision::EighthDotted, "1/8d"),
    (NoteDivision::EighthTriplet, "1/8t"),
    (NoteDivision::Sixteenth, "1/16"),
    (NoteDivision::SixteenthDotted, "1/16d"),
    (NoteDivision::SixteenthTriplet, "1/16t"),
];

#[cfg(test)]
#[inline]
pub(crate) fn modulation_controls_enabled(mode: DelayMode) -> bool {
    mode == DelayMode::Analog
}

#[cfg(test)]
#[inline]
pub(crate) fn note_division_selector_enabled(tempo_sync: bool) -> bool {
    tempo_sync
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

                            ui.add_space(4.0 * scale);
                            let mut sync_value = params.tempo_sync.value();
                            handle_bool_param(
                                ui,
                                setter,
                                &params.tempo_sync,
                                &mut sync_value,
                                "Tempo Sync",
                            );

                            ui.add_space(4.0 * scale);
                            handle_enum_combobox::<NoteDivision>(
                                ui,
                                setter,
                                &params.note_division,
                                "Div",
                                "note_div",
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

                            ui.add_space(4.0 * scale);
                            handle_enum_buttons::<DelayMode>(
                                ui,
                                setter,
                                &params.mode,
                                "Mode",
                            );
                        });

                        // STEREO section
                        cols[2].group(|ui| {
                            section_label(ui, "STEREO", ACCENT_NAVY, scale);

                            let mut ping_pong_value = params.ping_pong.value();
                            handle_bool_param(
                                ui,
                                setter,
                                &params.ping_pong,
                                &mut ping_pong_value,
                                "Ping Pong",
                            );

                            ui.add_space(4.0 * scale);
                            let mut offset_value = params.stereo_offset.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.stereo_offset,
                                &mut offset_value,
                                "Offset (ms)",
                                0.0..=50.0,
                            );
                        });
                    });

                    ui.add_space(8.0 * scale);

                    // ── Row 2: TONE | MODULATION | DUCK ──
                    ui.columns(3, |cols| {
                        // TONE section
                        cols[0].group(|ui| {
                            section_label(ui, "TONE", TRIBAL_BROWN, scale);

                            let mut hp_value = params.highpass_freq.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.highpass_freq,
                                &mut hp_value,
                                "HP (Hz)",
                                20.0..=1000.0,
                            );

                            ui.add_space(4.0 * scale);
                            let mut lp_value = params.lowpass_freq.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.lowpass_freq,
                                &mut lp_value,
                                "LP (Hz)",
                                500.0..=20000.0,
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

                            ui.add_space(4.0 * scale);
                            let mut rate_value = params.mod_rate.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.mod_rate,
                                &mut rate_value,
                                "Rate (Hz)",
                                0.1..=5.0,
                            );

                            ui.add_space(4.0 * scale);
                            let mut depth_value = params.mod_depth.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.mod_depth,
                                &mut depth_value,
                                "Depth",
                                0.0..=1.0,
                            );

                            ui.add_space(4.0 * scale);
                            let mut drive_value = params.drive.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.drive,
                                &mut drive_value,
                                "Drive",
                                0.0..=1.0,
                            );
                        });

                        // DUCK section
                        cols[2].group(|ui| {
                            section_label(ui, "DUCK", DOVE_GOLD, scale);

                            let mut duck_amt_value = params.duck_amount.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.duck_amount,
                                &mut duck_amt_value,
                                "Amount",
                                0.0..=1.0,
                            );

                            ui.add_space(4.0 * scale);
                            let mut duck_thr_value = params.duck_threshold.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.duck_threshold,
                                &mut duck_thr_value,
                                "Threshold",
                                0.0..=1.0,
                            );
                        });
                    });
                });
        },
    )
}
