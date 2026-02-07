//! GUI editor for GenX Delay — Woodstock 99 theme

use nih_plug::prelude::*;
use nih_plug_egui::egui;
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct WoodstockDecorationMetrics {
    pub(crate) barbed_wire_opacity: f32,
    pub(crate) tribal_opacity: f32,
    pub(crate) dove_opacity: f32,
    pub(crate) speckle_opacity: f32,
    pub(crate) rust_stamp_opacity: f32,
    pub(crate) wire_spacing: f32,
    pub(crate) corner_extent: f32,
    pub(crate) dove_size: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SectionAccentMetrics {
    pub(crate) line_thickness: f32,
    pub(crate) ornament_radius: f32,
    pub(crate) ornament_gap: f32,
}

pub(crate) fn woodstock_decoration_metrics(scale: f32) -> WoodstockDecorationMetrics {
    let clamped_scale = scale.clamp(0.5, 2.0);
    WoodstockDecorationMetrics {
        // Keep decorative overlays subtle per icon-pack guidance (6%..18%).
        barbed_wire_opacity: (0.13 + 0.02 * clamped_scale).clamp(0.06, 0.18),
        tribal_opacity: (0.10 + 0.02 * clamped_scale).clamp(0.06, 0.18),
        dove_opacity: (0.12 + 0.015 * clamped_scale).clamp(0.06, 0.18),
        speckle_opacity: (0.055 + 0.01 * clamped_scale).clamp(0.06, 0.12),
        rust_stamp_opacity: (0.09 + 0.02 * clamped_scale).clamp(0.06, 0.18),
        wire_spacing: 24.0 * clamped_scale,
        corner_extent: 34.0 * clamped_scale,
        dove_size: 28.0 * clamped_scale,
    }
}

pub(crate) fn section_accent_metrics(scale: f32) -> SectionAccentMetrics {
    let clamped_scale = scale.clamp(0.5, 2.0);
    SectionAccentMetrics {
        line_thickness: (0.9 * clamped_scale).max(0.7),
        ornament_radius: (1.2 * clamped_scale).max(0.9),
        ornament_gap: 7.0 * clamped_scale,
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct ResizeGeometryMetrics {
    pub(crate) content_scale: f32,
    pub(crate) barbed_wire_margin: f32,
    pub(crate) tribal_margin: f32,
    pub(crate) tribal_outer_stroke: f32,
    pub(crate) tribal_inner_stroke: f32,
    pub(crate) rust_stamp_outer_stroke: f32,
}

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
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::proportional(24.0 * scale),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::proportional(11.0 * scale),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::proportional(11.0 * scale),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::monospace(11.0 * scale),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::proportional(9.0 * scale),
    );

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
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).small().color(TEXT_DARK));
        ui.label(
            egui::RichText::new(format!("{value:.2}"))
                .small()
                .monospace()
                .color(TRIBAL_BROWN),
        );
    });

    let response = ui.add(egui::Slider::new(value, range).show_value(false));
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
    draw_section_accent(ui, color, scale);
    ui.add_space(4.0 * scale);
}

fn draw_section_accent(ui: &mut egui::Ui, color: egui::Color32, scale: f32) {
    let metrics = section_accent_metrics(scale);
    let width = ui.available_width().max(40.0 * scale);
    let (response, painter) =
        ui.allocate_painter(egui::vec2(width, 6.0 * scale), egui::Sense::hover());

    let rect = response.rect;
    let y = rect.center().y;
    let left = rect.left() + 2.0 * scale;
    let right = rect.right() - 2.0 * scale;
    let center = rect.center().x;

    painter.line_segment(
        [egui::pos2(left, y), egui::pos2(right, y)],
        egui::Stroke::new(metrics.line_thickness, color.gamma_multiply(0.42)),
    );
    painter.circle_filled(
        egui::pos2(center - metrics.ornament_gap, y),
        metrics.ornament_radius,
        color.gamma_multiply(0.52),
    );
    painter.circle_filled(
        egui::pos2(center + metrics.ornament_gap, y),
        metrics.ornament_radius,
        color.gamma_multiply(0.52),
    );
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(center, y), egui::vec2(4.4 * scale, 1.4 * scale)),
        0.0,
        color.gamma_multiply(0.62),
    );
}

fn draw_time_section(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    params: &Arc<GenXDelayParams>,
    scale: f32,
) {
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
    handle_bool_param(ui, setter, &params.reverse, &mut reverse_value, "Reverse");

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
    handle_enum_combobox::<NoteDivision>(ui, setter, &params.note_division, "Div", "note_div");
}

fn draw_main_section(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    params: &Arc<GenXDelayParams>,
    scale: f32,
) {
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
    handle_slider_param(ui, setter, &params.mix, &mut mix_value, "Mix", 0.0..=1.0);

    ui.add_space(4.0 * scale);
    handle_enum_buttons::<DelayMode>(ui, setter, &params.mode, "Mode");
}

fn draw_stereo_section(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    params: &Arc<GenXDelayParams>,
    scale: f32,
) {
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
}

fn draw_tone_section(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    params: &Arc<GenXDelayParams>,
    scale: f32,
) {
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
}

fn draw_modulation_section(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    params: &Arc<GenXDelayParams>,
    scale: f32,
    modulation_enabled: bool,
) {
    let modulation_label_color = if modulation_enabled {
        RUST
    } else {
        RUST.gamma_multiply(0.55)
    };
    section_label(ui, "MODULATION", modulation_label_color, scale);
    ui.label(
        egui::RichText::new("(Analog only)")
            .small()
            .color(modulation_label_color),
    );

    ui.add_enabled_ui(modulation_enabled, |ui| {
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
}

fn draw_duck_section(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    params: &Arc<GenXDelayParams>,
    scale: f32,
) {
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
}

fn draw_section_by_index(
    ui: &mut egui::Ui,
    section_index: usize,
    setter: &ParamSetter<'_>,
    params: &Arc<GenXDelayParams>,
    scale: f32,
    modulation_enabled: bool,
) {
    ui.group(|ui| match section_index {
        0 => draw_time_section(ui, setter, params, scale),
        1 => draw_main_section(ui, setter, params, scale),
        2 => draw_stereo_section(ui, setter, params, scale),
        3 => draw_tone_section(ui, setter, params, scale),
        4 => draw_modulation_section(ui, setter, params, scale, modulation_enabled),
        5 => draw_duck_section(ui, setter, params, scale),
        _ => {}
    });
}

fn draw_adaptive_control_grid(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    params: &Arc<GenXDelayParams>,
    scale: f32,
    modulation_enabled: bool,
) {
    let width = ui.available_width();
    let columns = if width >= 840.0 {
        3
    } else if width >= 560.0 {
        2
    } else {
        1
    };
    let section_order: [usize; 6] = [0, 1, 2, 3, 4, 5];

    for chunk in section_order.chunks(columns) {
        ui.columns(columns, |cols| {
            for (col_idx, col_ui) in cols.iter_mut().enumerate() {
                if let Some(section_index) = chunk.get(col_idx) {
                    draw_section_by_index(
                        col_ui,
                        *section_index,
                        setter,
                        params,
                        scale,
                        modulation_enabled,
                    );
                }
            }
        });
        ui.add_space(8.0 * scale);
    }
}

fn draw_dove_mark(painter: &egui::Painter, top_left: egui::Pos2, size: f32, color: egui::Color32) {
    let point = |x: f32, y: f32| egui::pos2(top_left.x + x * size, top_left.y + y * size);

    let wing = vec![
        point(0.02, 0.44),
        point(0.34, 0.02),
        point(0.72, 0.22),
        point(0.46, 0.56),
    ];
    let body = vec![
        point(0.30, 0.48),
        point(0.58, 0.30),
        point(0.88, 0.46),
        point(0.54, 0.80),
        point(0.24, 0.72),
    ];
    let beak = vec![point(0.86, 0.44), point(0.98, 0.39), point(0.89, 0.52)];

    painter.add(egui::Shape::convex_polygon(wing, color, egui::Stroke::NONE));
    painter.add(egui::Shape::convex_polygon(
        body,
        color.gamma_multiply(0.95),
        egui::Stroke::NONE,
    ));
    painter.add(egui::Shape::convex_polygon(
        beak,
        ACCENT_OLIVE.gamma_multiply(0.3),
        egui::Stroke::NONE,
    ));

    let branch_stroke =
        egui::Stroke::new((size * 0.05).max(0.8), ACCENT_OLIVE.gamma_multiply(0.25));
    painter.line_segment([point(0.66, 0.74), point(1.05, 0.90)], branch_stroke);
    painter.line_segment([point(0.80, 0.80), point(0.88, 0.70)], branch_stroke);
    painter.line_segment([point(0.91, 0.86), point(0.98, 0.77)], branch_stroke);
}

fn draw_grunge_speckles(
    painter: &egui::Painter,
    rect: egui::Rect,
    scale: f32,
    color: egui::Color32,
) {
    let area = rect.width() * rect.height();
    let count = ((area / 2500.0).round() as usize).clamp(60, 170);
    for i in 0..count {
        let seed = (i as u32)
            .wrapping_mul(747_796_405)
            .wrapping_add(2_891_336_453);
        let x_hash = mix_u32(seed);
        let y_hash = mix_u32(seed.wrapping_add(0x9E37_79B9));
        let r_hash = mix_u32(seed.wrapping_add(0x85EB_CA6B));

        let fx = x_hash as f32 / u32::MAX as f32;
        let fy = y_hash as f32 / u32::MAX as f32;
        let radius = (0.35 + ((r_hash & 0xff) as f32 / 255.0) * 1.2) * scale;
        let point = egui::pos2(
            rect.left() + fx * rect.width(),
            rect.top() + fy * rect.height(),
        );
        painter.circle_filled(point, radius, color);
    }
}

#[inline]
fn mix_u32(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7FEB_352D);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846C_A68B);
    x ^ (x >> 16)
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

#[inline]
pub(crate) fn modulation_controls_enabled(mode: DelayMode) -> bool {
    mode == DelayMode::Analog
}

#[cfg(test)]
#[inline]
pub(crate) fn note_division_selector_enabled(tempo_sync: bool) -> bool {
    tempo_sync
}

#[inline]
fn content_scale_from_dimensions(width: f32, height: f32) -> f32 {
    let sx = width / WINDOW_WIDTH as f32;
    let sy = height / WINDOW_HEIGHT as f32;
    sx.min(sy).max(0.5) // clamp to prevent illegibly small text
}

fn content_scale_from_rect(rect: egui::Rect) -> f32 {
    content_scale_from_dimensions(rect.width(), rect.height())
}

fn scaled_margin(scale: f32, margin: f32) -> f32 {
    margin * scale
}

#[cfg(test)]
pub(crate) fn resize_geometry_metrics_for_dimensions(
    width: f32,
    height: f32,
) -> ResizeGeometryMetrics {
    let content_scale = content_scale_from_dimensions(width, height);
    ResizeGeometryMetrics {
        content_scale,
        barbed_wire_margin: scaled_margin(content_scale.max(1.0), 14.0),
        tribal_margin: 6.0 * content_scale,
        tribal_outer_stroke: 1.6 * content_scale,
        tribal_inner_stroke: 1.0 * content_scale,
        rust_stamp_outer_stroke: 1.1 * content_scale,
    }
}

fn draw_woodstock_decorations(ui: &egui::Ui, rect: egui::Rect, scale: f32) {
    let painter = ui.painter();
    let metrics = woodstock_decoration_metrics(scale);

    draw_barbed_wire_line(
        painter,
        rect,
        rect.top() + 13.0 * scale,
        metrics.wire_spacing,
        RUST.gamma_multiply(metrics.barbed_wire_opacity),
        1.0 * scale,
    );
    draw_barbed_wire_line(
        painter,
        rect,
        rect.bottom() - 12.0 * scale,
        metrics.wire_spacing,
        RUST.gamma_multiply(metrics.barbed_wire_opacity),
        1.0 * scale,
    );

    draw_tribal_corner(
        painter,
        rect,
        egui::Align2::LEFT_TOP,
        metrics.corner_extent,
        TRIBAL_BROWN.gamma_multiply(metrics.tribal_opacity),
        scale,
    );
    draw_tribal_corner(
        painter,
        rect,
        egui::Align2::RIGHT_TOP,
        metrics.corner_extent,
        TRIBAL_BROWN.gamma_multiply(metrics.tribal_opacity),
        scale,
    );
    draw_tribal_corner(
        painter,
        rect,
        egui::Align2::LEFT_BOTTOM,
        metrics.corner_extent,
        TRIBAL_BROWN.gamma_multiply(metrics.tribal_opacity),
        scale,
    );
    draw_tribal_corner(
        painter,
        rect,
        egui::Align2::RIGHT_BOTTOM,
        metrics.corner_extent,
        TRIBAL_BROWN.gamma_multiply(metrics.tribal_opacity),
        scale,
    );

    let dove_pos = egui::pos2(
        rect.right() - metrics.dove_size - (16.0 * scale),
        rect.top() + (18.0 * scale),
    );
    draw_dove_mark(
        painter,
        dove_pos,
        metrics.dove_size,
        DOVE_GOLD.gamma_multiply(metrics.dove_opacity),
    );

    let stamp_center = egui::pos2(rect.left() + (28.0 * scale), rect.bottom() - (30.0 * scale));
    draw_rust_stamp(
        painter,
        stamp_center,
        12.0 * scale,
        RUST.gamma_multiply(metrics.rust_stamp_opacity),
        scale,
    );

    draw_grunge_speckles(
        painter,
        rect.shrink(8.0 * scale),
        0.9 * scale,
        TEXT_DARK.gamma_multiply(metrics.speckle_opacity),
    );
}

fn draw_barbed_wire_line(
    painter: &egui::Painter,
    rect: egui::Rect,
    y: f32,
    spacing: f32,
    color: egui::Color32,
    stroke_width: f32,
) {
    let stroke = egui::Stroke::new(stroke_width, color);
    let start_x = rect.left() + scaled_margin(stroke_width.max(1.0), 14.0);
    let end_x = rect.right() - scaled_margin(stroke_width.max(1.0), 14.0);
    painter.line_segment([egui::pos2(start_x, y), egui::pos2(end_x, y)], stroke);

    let mut x = start_x + spacing * 0.5;
    while x < end_x - spacing * 0.5 {
        let barb = 3.2 * stroke_width.max(1.0);
        painter.line_segment(
            [
                egui::pos2(x - barb, y - barb),
                egui::pos2(x + barb, y + barb),
            ],
            stroke,
        );
        painter.line_segment(
            [
                egui::pos2(x - barb, y + barb),
                egui::pos2(x + barb, y - barb),
            ],
            stroke,
        );
        painter.circle_stroke(
            egui::pos2(x - 4.2 * stroke_width, y),
            1.4 * stroke_width,
            stroke,
        );
        painter.circle_stroke(
            egui::pos2(x + 4.2 * stroke_width, y),
            1.4 * stroke_width,
            stroke,
        );
        x += spacing;
    }
}

fn draw_tribal_corner(
    painter: &egui::Painter,
    rect: egui::Rect,
    corner: egui::Align2,
    extent: f32,
    color: egui::Color32,
    scale: f32,
) {
    let margin = 6.0 * scale;
    let (anchor, sx, sy) = match corner {
        egui::Align2::LEFT_TOP => (
            egui::pos2(rect.left() + margin, rect.top() + margin),
            1.0,
            1.0,
        ),
        egui::Align2::RIGHT_TOP => (
            egui::pos2(rect.right() - margin, rect.top() + margin),
            -1.0,
            1.0,
        ),
        egui::Align2::LEFT_BOTTOM => (
            egui::pos2(rect.left() + margin, rect.bottom() - margin),
            1.0,
            -1.0,
        ),
        egui::Align2::RIGHT_BOTTOM => (
            egui::pos2(rect.right() - margin, rect.bottom() - margin),
            -1.0,
            -1.0,
        ),
        _ => return,
    };

    let map_point = |x: f32, y: f32| egui::pos2(anchor.x + sx * x, anchor.y + sy * y);
    let outer = vec![
        map_point(0.0, 0.0),
        map_point(extent * 0.32, extent * 0.06),
        map_point(extent * 0.56, extent * 0.24),
        map_point(extent * 0.76, extent * 0.50),
    ];
    let inner = vec![
        map_point(extent * 0.09, extent * 0.16),
        map_point(extent * 0.32, extent * 0.24),
        map_point(extent * 0.50, extent * 0.41),
    ];

    painter.add(egui::Shape::line(
        outer,
        egui::Stroke::new(1.6 * scale, color),
    ));
    painter.add(egui::Shape::line(
        inner,
        egui::Stroke::new(1.0 * scale, color.gamma_multiply(0.75)),
    ));
}

fn draw_rust_stamp(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    color: egui::Color32,
    scale: f32,
) {
    let stroke = egui::Stroke::new(1.1 * scale, color);
    painter.circle_stroke(center, radius, stroke);
    painter.circle_stroke(
        center,
        radius * 0.66,
        egui::Stroke::new(0.9 * scale, color.gamma_multiply(0.8)),
    );
    painter.line_segment(
        [
            egui::pos2(center.x - radius * 0.44, center.y - radius * 0.2),
            egui::pos2(center.x + radius * 0.5, center.y + radius * 0.3),
        ],
        egui::Stroke::new(0.9 * scale, color.gamma_multiply(0.75)),
    );
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
            let viewport_size = egui_ctx.screen_rect().size();
            let initial_scale = content_scale_from_dimensions(viewport_size.x, viewport_size.y);
            apply_theme(egui_ctx, initial_scale);
            let current_mode = params.mode.value();
            let modulation_enabled = modulation_controls_enabled(current_mode);

            egui::CentralPanel::default().show(egui_ctx, |ui| {
                let ui_rect = ui.max_rect();
                let scale = content_scale_from_rect(ui_rect);
                apply_theme(egui_ctx, scale);
                draw_woodstock_decorations(ui, ui_rect, scale);

                // ── Header ──
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("GENX DELAY").heading().color(TEXT_DARK));
                    ui.label(
                        egui::RichText::new("— WOODSTOCK 99 —")
                            .small()
                            .color(TRIBAL_BROWN),
                    );
                });

                ui.add_space(8.0 * scale);
                draw_adaptive_control_grid(ui, setter, &params, scale, modulation_enabled);
            });
        },
    )
}
