//! GUI editor for GenX Delay — Woodstock 99 theme

use nih_plug::prelude::*;
use nih_plug_egui::egui;
use nih_plug_egui::resizable_window::ResizableWindow;
use nih_plug_egui::{create_egui_editor, EguiState};
use std::sync::Arc;

use crate::GenXDelayParams;

const WINDOW_WIDTH: u32 = 300;
const WINDOW_HEIGHT: u32 = 200;

// Woodstock 99 color palette
const BG_MAIN: egui::Color32 = egui::Color32::from_rgb(235, 228, 215);
const BG_PANEL: egui::Color32 = egui::Color32::from_rgb(215, 205, 190);
const TEXT_DARK: egui::Color32 = egui::Color32::from_rgb(45, 40, 35);
const TRIBAL_BROWN: egui::Color32 = egui::Color32::from_rgb(75, 55, 40);
const ACCENT_WARM: egui::Color32 = egui::Color32::from_rgb(180, 95, 65);
const ACCENT_OLIVE: egui::Color32 = egui::Color32::from_rgb(105, 115, 80);

pub fn default_state() -> Arc<EguiState> {
    EguiState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT)
}

fn apply_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();

    // Background
    visuals.panel_fill = BG_MAIN;
    visuals.window_fill = BG_MAIN;
    visuals.extreme_bg_color = BG_PANEL;

    // Widget colors
    visuals.widgets.inactive.bg_fill = BG_PANEL;
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, TEXT_DARK);
    visuals.widgets.inactive.weak_bg_fill = BG_PANEL;

    visuals.widgets.hovered.bg_fill = ACCENT_OLIVE;
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, BG_MAIN);
    visuals.widgets.hovered.weak_bg_fill = ACCENT_OLIVE;

    visuals.widgets.active.bg_fill = ACCENT_OLIVE;
    visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0, BG_MAIN);
    visuals.widgets.active.weak_bg_fill = ACCENT_OLIVE;

    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, TEXT_DARK);
    visuals.widgets.noninteractive.bg_fill = BG_MAIN;

    // Slider rail
    visuals.selection.bg_fill = ACCENT_OLIVE;
    visuals.selection.stroke = egui::Stroke::new(1.0, ACCENT_OLIVE);

    // Window rounding and borders
    visuals.window_stroke = egui::Stroke::new(1.0, TRIBAL_BROWN);

    ctx.set_visuals(visuals);

    // Font sizes
    let mut style = (*ctx.style()).clone();
    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::proportional(24.0));
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(11.0));
    style
        .text_styles
        .insert(egui::TextStyle::Small, egui::FontId::proportional(9.0));
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
            apply_theme(egui_ctx);

            ResizableWindow::new("genx_delay")
                .min_size([300.0, 200.0])
                .show(egui_ctx, &state_for_resize, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("GENX DELAY").heading().color(TEXT_DARK));
                        ui.label(
                            egui::RichText::new("— WOODSTOCK 99 —")
                                .small()
                                .color(TRIBAL_BROWN),
                        );
                    });

                    ui.add_space(12.0);

                    ui.columns(2, |columns| {
                        columns[0].group(|ui| {
                            ui.label(egui::RichText::new("TIME").small().color(ACCENT_WARM));
                            ui.add_space(6.0);

                            let mut delay_time_value = params.delay_time.value();
                            handle_slider_param(
                                ui,
                                setter,
                                &params.delay_time,
                                &mut delay_time_value,
                                "Delay Time (ms)",
                                1.0..=2500.0,
                            );

                            ui.add_space(4.0);
                            let mut reverse_value = params.reverse.value();
                            handle_bool_param(
                                ui,
                                setter,
                                &params.reverse,
                                &mut reverse_value,
                                "Reverse",
                            );
                        });

                        columns[1].group(|ui| {
                            ui.label(egui::RichText::new("MAIN").small().color(ACCENT_OLIVE));
                            ui.add_space(6.0);

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
                        });
                    });
                });
        },
    )
}
