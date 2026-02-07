//! GUI editor for GenX Delay — Woodstock 99 theme (1969 poster aesthetic)

use nih_plug::prelude::*;
use nih_plug_egui::egui;
use nih_plug_egui::{create_egui_editor, EguiState};
use std::sync::Arc;

use crate::{DelayMode, GenXDelayParams, NoteDivision};

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 420;

// 1969 Woodstock poster color palette — crimson red background, cream text
const BG_CRIMSON: egui::Color32 = egui::Color32::from_rgb(183, 28, 28);
const BG_PANEL_DARK: egui::Color32 = egui::Color32::from_rgb(153, 23, 23);
const TEXT_CREAM: egui::Color32 = egui::Color32::from_rgb(255, 248, 235);
const POSTER_WHITE: egui::Color32 = egui::Color32::from_rgb(240, 235, 225);
const ACCENT_CORAL: egui::Color32 = egui::Color32::from_rgb(255, 167, 130);
const ACCENT_SAGE: egui::Color32 = egui::Color32::from_rgb(180, 210, 160);
const ACCENT_SKY: egui::Color32 = egui::Color32::from_rgb(160, 195, 230);
const ACCENT_AMBER: egui::Color32 = egui::Color32::from_rgb(255, 200, 120);
const DOVE_WHITE: egui::Color32 = egui::Color32::from_rgb(255, 252, 245);
const ACCENT_TONE: egui::Color32 = egui::Color32::from_rgb(220, 190, 170);
const HIGHLIGHT_GOLD: egui::Color32 = egui::Color32::from_rgb(255, 215, 140);

#[cfg(test)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WoodstockDecorationMetrics {
    pub(crate) vine_border_opacity: f32,
    pub(crate) flower_corner_opacity: f32,
    pub(crate) dove_opacity: f32,
    pub(crate) starfield_opacity: f32,
    pub(crate) peace_symbol_opacity: f32,
    pub(crate) vine_spacing: f32,
    pub(crate) corner_extent: f32,
    pub(crate) dove_size: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SectionAccentMetrics {
    pub(crate) line_thickness: f32,
    pub(crate) ornament_radius: f32,
    pub(crate) ornament_gap: f32,
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn woodstock_decoration_metrics(scale: f32) -> WoodstockDecorationMetrics {
    let clamped_scale = scale.clamp(0.5, 2.0);
    WoodstockDecorationMetrics {
        // Keep decorative overlays subtle (6%..18%).
        vine_border_opacity: (0.13 + 0.02 * clamped_scale).clamp(0.06, 0.18),
        flower_corner_opacity: (0.10 + 0.02 * clamped_scale).clamp(0.06, 0.18),
        dove_opacity: (0.12 + 0.015 * clamped_scale).clamp(0.06, 0.18),
        starfield_opacity: (0.055 + 0.01 * clamped_scale).clamp(0.06, 0.12),
        peace_symbol_opacity: (0.09 + 0.02 * clamped_scale).clamp(0.06, 0.18),
        vine_spacing: 24.0 * clamped_scale,
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
    pub(crate) vine_border_margin: f32,
    pub(crate) flower_corner_margin: f32,
    pub(crate) flower_outer_stroke: f32,
    pub(crate) flower_inner_stroke: f32,
    pub(crate) peace_symbol_stroke: f32,
}

pub fn default_state() -> Arc<EguiState> {
    EguiState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT)
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Righteous — retro 60s/70s display font for the heading (1969 Woodstock poster feel)
    fonts.font_data.insert(
        "Righteous".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/Righteous-Regular.ttf"
        ))),
    );

    // Josefin Sans — clean vintage sans-serif for body text
    fonts.font_data.insert(
        "JosefinSans".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/JosefinSans-Variable.ttf"
        ))),
    );

    // Set Josefin Sans as the primary proportional font (body, labels, buttons)
    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        family.insert(0, "JosefinSans".to_owned());
    }

    // Register a named "Righteous" family for the heading, with Josefin Sans fallback
    fonts.families.insert(
        egui::FontFamily::Name("Righteous".into()),
        vec!["Righteous".to_owned(), "JosefinSans".to_owned()],
    );

    ctx.set_fonts(fonts);
}

fn apply_theme(ctx: &egui::Context, scale: f32) {
    let mut visuals = egui::Visuals::dark();

    // Background — deep crimson
    visuals.panel_fill = BG_CRIMSON;
    visuals.window_fill = BG_CRIMSON;
    visuals.extreme_bg_color = BG_PANEL_DARK;

    // Widget colors — cream text on dark crimson panels
    visuals.widgets.inactive.bg_fill = BG_PANEL_DARK;
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0 * scale, TEXT_CREAM);
    visuals.widgets.inactive.weak_bg_fill = BG_PANEL_DARK;

    visuals.widgets.hovered.bg_fill = HIGHLIGHT_GOLD;
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5 * scale, BG_CRIMSON);
    visuals.widgets.hovered.weak_bg_fill = HIGHLIGHT_GOLD;

    visuals.widgets.active.bg_fill = ACCENT_SAGE;
    visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0 * scale, BG_CRIMSON);
    visuals.widgets.active.weak_bg_fill = ACCENT_SAGE;

    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0 * scale, TEXT_CREAM);
    visuals.widgets.noninteractive.bg_fill = BG_CRIMSON;

    // Slider rail — sage green selection on crimson
    visuals.selection.bg_fill = ACCENT_SAGE;
    visuals.selection.stroke = egui::Stroke::new(1.0 * scale, ACCENT_SAGE);

    // Window border — darker wine
    visuals.window_stroke = egui::Stroke::new(1.0 * scale, egui::Color32::from_rgb(120, 18, 18));

    ctx.set_visuals(visuals);

    // Font sizes — scale proportionally with window
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(24.0 * scale, egui::FontFamily::Name("Righteous".into())),
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
        ui.label(egui::RichText::new(label).small().color(TEXT_CREAM));
        ui.label(
            egui::RichText::new(format!("{value:.2}"))
                .small()
                .monospace()
                .color(POSTER_WHITE),
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
        ui.label(egui::RichText::new(label).small().color(TEXT_CREAM));
        let current = param.value();
        for (idx, name) in T::variants().iter().enumerate() {
            let variant = T::from_index(idx);
            let is_selected = current == variant;
            let text_color = if is_selected {
                BG_PANEL_DARK
            } else {
                POSTER_WHITE
            };
            let button_text = egui::RichText::new(*name).color(text_color);
            if ui.selectable_label(is_selected, button_text).clicked() {
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
        ui.label(egui::RichText::new(label).small().color(TEXT_CREAM));
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

    // Vine-like segments with a gentle arc
    painter.line_segment(
        [
            egui::pos2(left, y),
            egui::pos2(center - metrics.ornament_gap * 1.5, y - 0.5 * scale),
        ],
        egui::Stroke::new(metrics.line_thickness, color.gamma_multiply(0.42)),
    );
    painter.line_segment(
        [
            egui::pos2(center + metrics.ornament_gap * 1.5, y - 0.5 * scale),
            egui::pos2(right, y),
        ],
        egui::Stroke::new(metrics.line_thickness, color.gamma_multiply(0.42)),
    );

    // Flower bud circles with outer petal ring
    let bud_left = egui::pos2(center - metrics.ornament_gap, y);
    let bud_right = egui::pos2(center + metrics.ornament_gap, y);
    painter.circle_filled(
        bud_left,
        metrics.ornament_radius,
        color.gamma_multiply(0.52),
    );
    painter.circle_filled(
        bud_right,
        metrics.ornament_radius,
        color.gamma_multiply(0.52),
    );
    painter.circle_stroke(
        bud_left,
        metrics.ornament_radius * 1.5,
        egui::Stroke::new(0.5 * scale, color.gamma_multiply(0.30)),
    );
    painter.circle_stroke(
        bud_right,
        metrics.ornament_radius * 1.5,
        egui::Stroke::new(0.5 * scale, color.gamma_multiply(0.30)),
    );

    // Center leaf diamond
    let leaf_half_w = 2.5 * scale;
    let leaf_half_h = 1.0 * scale;
    let leaf_points = vec![
        egui::pos2(center, y - leaf_half_h),
        egui::pos2(center + leaf_half_w, y),
        egui::pos2(center, y + leaf_half_h),
        egui::pos2(center - leaf_half_w, y),
    ];
    painter.add(egui::Shape::convex_polygon(
        leaf_points,
        color.gamma_multiply(0.55),
        egui::Stroke::NONE,
    ));
}

fn draw_time_section(
    ui: &mut egui::Ui,
    setter: &ParamSetter<'_>,
    params: &Arc<GenXDelayParams>,
    scale: f32,
) {
    section_label(ui, "TIME", ACCENT_CORAL, scale);

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
    section_label(ui, "MAIN", ACCENT_SAGE, scale);

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
    section_label(ui, "STEREO", ACCENT_SKY, scale);

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
    section_label(ui, "TONE", ACCENT_TONE, scale);

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
        ACCENT_AMBER
    } else {
        ACCENT_AMBER.gamma_multiply(0.45)
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
    section_label(ui, "DUCK", DOVE_WHITE, scale);

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
    let columns = if width >= 540.0 {
        3
    } else if width >= 360.0 {
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
        ui.add_space(4.0 * scale);
    }
}

// ── 1969 Poster Decoration Functions ──

/// Dove perched on a guitar neck — the iconic 1969 Woodstock poster silhouette.
#[cfg(test)]
#[allow(dead_code)]
fn draw_dove_on_guitar(
    painter: &egui::Painter,
    top_left: egui::Pos2,
    size: f32,
    color: egui::Color32,
) {
    let point = |x: f32, y: f32| egui::pos2(top_left.x + x * size, top_left.y + y * size);

    // Guitar neck (horizontal bar the dove sits on)
    painter.rect_filled(
        egui::Rect::from_min_max(point(0.0, 0.55), point(1.0, 0.63)),
        0.0,
        color.gamma_multiply(0.7),
    );
    // Fret lines
    let fret_stroke = egui::Stroke::new((size * 0.02).max(0.5), color.gamma_multiply(0.4));
    for fret_x in [0.25, 0.50, 0.75] {
        painter.line_segment([point(fret_x, 0.55), point(fret_x, 0.63)], fret_stroke);
    }
    // Headstock
    let headstock = vec![
        point(0.88, 0.53),
        point(0.95, 0.50),
        point(1.0, 0.53),
        point(1.0, 0.65),
        point(0.95, 0.68),
        point(0.88, 0.65),
    ];
    painter.add(egui::Shape::convex_polygon(
        headstock,
        color.gamma_multiply(0.65),
        egui::Stroke::NONE,
    ));
    // Tuning pegs
    for py in [0.54, 0.58, 0.62] {
        painter.circle_filled(point(0.97, py), size * 0.015, color.gamma_multiply(0.5));
    }

    // Dove body sitting on the neck
    let body = vec![
        point(0.25, 0.42),
        point(0.40, 0.35),
        point(0.52, 0.40),
        point(0.50, 0.55),
        point(0.28, 0.55),
    ];
    painter.add(egui::Shape::convex_polygon(
        body,
        color.gamma_multiply(0.95),
        egui::Stroke::NONE,
    ));

    // Raised wing (sweeping upward, 1969 poster style)
    let wing = vec![
        point(0.20, 0.42),
        point(0.30, 0.10),
        point(0.55, 0.18),
        point(0.48, 0.40),
    ];
    painter.add(egui::Shape::convex_polygon(wing, color, egui::Stroke::NONE));

    // Head
    painter.circle_filled(point(0.26, 0.36), size * 0.04, color.gamma_multiply(0.95));
    // Beak
    let beak = vec![point(0.20, 0.34), point(0.15, 0.36), point(0.22, 0.38)];
    painter.add(egui::Shape::convex_polygon(
        beak,
        color.gamma_multiply(0.8),
        egui::Stroke::NONE,
    ));

    // Tail feathers
    let tail_stroke = egui::Stroke::new((size * 0.03).max(0.6), color.gamma_multiply(0.7));
    painter.line_segment([point(0.50, 0.48), point(0.60, 0.42)], tail_stroke);
    painter.line_segment([point(0.50, 0.50), point(0.62, 0.48)], tail_stroke);
    painter.line_segment([point(0.50, 0.52), point(0.60, 0.54)], tail_stroke);
}

/// Scattered stars and dots — cosmic festival night sky.
#[cfg(test)]
#[allow(dead_code)]
fn draw_starfield(painter: &egui::Painter, rect: egui::Rect, scale: f32, color: egui::Color32) {
    let area = rect.width() * rect.height();
    // Fewer points than grunge speckles — 1969 is cleaner
    let count = ((area / 4000.0).round() as usize).clamp(40, 100);
    for i in 0..count {
        let seed = (i as u32)
            .wrapping_mul(747_796_405)
            .wrapping_add(2_891_336_453);
        let x_hash = mix_u32(seed);
        let y_hash = mix_u32(seed.wrapping_add(0x9E37_79B9));
        let r_hash = mix_u32(seed.wrapping_add(0x85EB_CA6B));
        let type_hash = mix_u32(seed.wrapping_add(0x517C_C1B7));

        let fx = x_hash as f32 / u32::MAX as f32;
        let fy = y_hash as f32 / u32::MAX as f32;
        let point = egui::pos2(
            rect.left() + fx * rect.width(),
            rect.top() + fy * rect.height(),
        );

        // Vary opacity per star
        let opacity_vary = 0.4 + 0.6 * ((r_hash & 0xff) as f32 / 255.0);
        let star_color = color.gamma_multiply(opacity_vary);

        if (type_hash & 0x03) == 0 {
            // 25% chance: 4-pointed cross star
            let arm = (0.8 + ((r_hash & 0xff) as f32 / 255.0) * 0.7) * scale;
            let star_stroke = egui::Stroke::new((0.4 * scale).max(0.4), star_color);
            painter.line_segment(
                [
                    egui::pos2(point.x - arm, point.y),
                    egui::pos2(point.x + arm, point.y),
                ],
                star_stroke,
            );
            painter.line_segment(
                [
                    egui::pos2(point.x, point.y - arm),
                    egui::pos2(point.x, point.y + arm),
                ],
                star_stroke,
            );
        } else {
            // 75% chance: simple dot
            let radius = (0.25 + ((r_hash & 0xff) as f32 / 255.0) * 0.6) * scale;
            painter.circle_filled(point, radius, star_color);
        }
    }
}

#[inline]
#[cfg(test)]
#[allow(dead_code)]
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

#[cfg(test)]
#[allow(dead_code)]
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
        vine_border_margin: scaled_margin(content_scale.max(1.0), 14.0),
        flower_corner_margin: 6.0 * content_scale,
        flower_outer_stroke: 1.6 * content_scale,
        flower_inner_stroke: 1.0 * content_scale,
        peace_symbol_stroke: 1.1 * content_scale,
    }
}

#[cfg(test)]
#[allow(dead_code)]
fn draw_woodstock_decorations(ui: &egui::Ui, rect: egui::Rect, scale: f32) {
    let painter = ui.painter();
    let metrics = woodstock_decoration_metrics(scale);

    draw_vine_border(
        painter,
        rect,
        rect.top() + 13.0 * scale,
        metrics.vine_spacing,
        POSTER_WHITE.gamma_multiply(metrics.vine_border_opacity),
        1.0 * scale,
    );
    draw_vine_border(
        painter,
        rect,
        rect.bottom() - 12.0 * scale,
        metrics.vine_spacing,
        POSTER_WHITE.gamma_multiply(metrics.vine_border_opacity),
        1.0 * scale,
    );

    draw_flower_corner(
        painter,
        rect,
        egui::Align2::LEFT_TOP,
        metrics.corner_extent,
        POSTER_WHITE.gamma_multiply(metrics.flower_corner_opacity),
        scale,
    );
    draw_flower_corner(
        painter,
        rect,
        egui::Align2::RIGHT_TOP,
        metrics.corner_extent,
        POSTER_WHITE.gamma_multiply(metrics.flower_corner_opacity),
        scale,
    );
    draw_flower_corner(
        painter,
        rect,
        egui::Align2::LEFT_BOTTOM,
        metrics.corner_extent,
        POSTER_WHITE.gamma_multiply(metrics.flower_corner_opacity),
        scale,
    );
    draw_flower_corner(
        painter,
        rect,
        egui::Align2::RIGHT_BOTTOM,
        metrics.corner_extent,
        POSTER_WHITE.gamma_multiply(metrics.flower_corner_opacity),
        scale,
    );

    let dove_pos = egui::pos2(
        rect.right() - metrics.dove_size - (16.0 * scale),
        rect.top() + (18.0 * scale),
    );
    draw_dove_on_guitar(
        painter,
        dove_pos,
        metrics.dove_size,
        DOVE_WHITE.gamma_multiply(metrics.dove_opacity),
    );

    let stamp_center = egui::pos2(rect.left() + (28.0 * scale), rect.bottom() - (30.0 * scale));
    draw_peace_symbol(
        painter,
        stamp_center,
        12.0 * scale,
        POSTER_WHITE.gamma_multiply(metrics.peace_symbol_opacity),
        scale,
    );

    draw_starfield(
        painter,
        rect.shrink(8.0 * scale),
        0.9 * scale,
        TEXT_CREAM.gamma_multiply(metrics.starfield_opacity),
    );
}

/// Organic vine line with leaf shapes — replaces barbed wire.
#[cfg(test)]
#[allow(dead_code)]
fn draw_vine_border(
    painter: &egui::Painter,
    rect: egui::Rect,
    y: f32,
    spacing: f32,
    color: egui::Color32,
    stroke_width: f32,
) {
    let start_x = rect.left() + scaled_margin(stroke_width.max(1.0), 14.0);
    let end_x = rect.right() - scaled_margin(stroke_width.max(1.0), 14.0);

    // Undulating vine stem (sine-wave approximation via segments)
    let segments = 24;
    let dx = (end_x - start_x) / segments as f32;
    let amplitude = 1.5 * stroke_width.max(1.0);
    let vine_stroke = egui::Stroke::new(stroke_width, color);
    for seg in 0..segments {
        let x0 = start_x + seg as f32 * dx;
        let x1 = start_x + (seg + 1) as f32 * dx;
        let t0 = seg as f32 / segments as f32;
        let t1 = (seg + 1) as f32 / segments as f32;
        let y0 = y + amplitude * (t0 * std::f32::consts::TAU * 2.0).sin();
        let y1 = y + amplitude * (t1 * std::f32::consts::TAU * 2.0).sin();
        painter.line_segment([egui::pos2(x0, y0), egui::pos2(x1, y1)], vine_stroke);
    }

    // Leaves at regular intervals
    let mut x = start_x + spacing * 0.5;
    while x < end_x - spacing * 0.5 {
        let t = (x - start_x) / (end_x - start_x);
        let vine_y = y + amplitude * (t * std::f32::consts::TAU * 2.0).sin();
        let leaf_len = 4.0 * stroke_width.max(1.0);
        let leaf_width = 1.5 * stroke_width.max(1.0);

        // Upward leaf
        let leaf_up = vec![
            egui::pos2(x, vine_y),
            egui::pos2(x - leaf_width, vine_y - leaf_len),
            egui::pos2(x - leaf_width * 0.2, vine_y - leaf_len * 1.1),
            egui::pos2(x + leaf_width * 0.5, vine_y - leaf_len * 0.4),
        ];
        painter.add(egui::Shape::convex_polygon(
            leaf_up,
            color.gamma_multiply(0.7),
            egui::Stroke::NONE,
        ));

        // Leaf vein
        painter.line_segment(
            [
                egui::pos2(x, vine_y),
                egui::pos2(x - leaf_width * 0.5, vine_y - leaf_len * 0.8),
            ],
            egui::Stroke::new(stroke_width * 0.4, color.gamma_multiply(0.4)),
        );

        x += spacing;
    }
}

/// Flower cluster in each corner — replaces tribal corner motifs.
#[cfg(test)]
#[allow(dead_code)]
fn draw_flower_corner(
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

    // Curved stem from corner (S-curve approximation)
    let stem = vec![
        map_point(0.0, 0.0),
        map_point(extent * 0.15, extent * 0.12),
        map_point(extent * 0.35, extent * 0.20),
        map_point(extent * 0.55, extent * 0.35),
        map_point(extent * 0.68, extent * 0.52),
    ];
    painter.add(egui::Shape::line(
        stem,
        egui::Stroke::new(1.6 * scale, color),
    ));

    // Inner stem tendril
    let tendril = vec![
        map_point(extent * 0.10, extent * 0.18),
        map_point(extent * 0.28, extent * 0.30),
        map_point(extent * 0.42, extent * 0.44),
    ];
    painter.add(egui::Shape::line(
        tendril,
        egui::Stroke::new(1.0 * scale, color.gamma_multiply(0.75)),
    ));

    // Flower at the end of the stem — 5 petals
    let flower_center = map_point(extent * 0.68, extent * 0.52);
    let petal_r = extent * 0.12;
    for i in 0..5 {
        let angle = (i as f32) * std::f32::consts::TAU / 5.0 - std::f32::consts::FRAC_PI_4;
        let tip_x = flower_center.x + petal_r * angle.cos();
        let tip_y = flower_center.y + petal_r * angle.sin();
        let side_angle_l = angle + 0.4;
        let side_angle_r = angle - 0.4;
        let side_r = petal_r * 0.45;
        let petal = vec![
            flower_center,
            egui::pos2(
                flower_center.x + side_r * side_angle_l.cos(),
                flower_center.y + side_r * side_angle_l.sin(),
            ),
            egui::pos2(tip_x, tip_y),
            egui::pos2(
                flower_center.x + side_r * side_angle_r.cos(),
                flower_center.y + side_r * side_angle_r.sin(),
            ),
        ];
        painter.add(egui::Shape::convex_polygon(
            petal,
            color.gamma_multiply(0.6),
            egui::Stroke::NONE,
        ));
    }
    painter.circle_filled(flower_center, petal_r * 0.25, color.gamma_multiply(0.8));

    // Small buds along the stem
    let bud1 = map_point(extent * 0.25, extent * 0.16);
    let bud2 = map_point(extent * 0.45, extent * 0.32);
    painter.circle_filled(bud1, 1.5 * scale, color.gamma_multiply(0.5));
    painter.circle_filled(bud2, 1.8 * scale, color.gamma_multiply(0.55));
}

/// Classic peace sign — replaces rust stamp.
#[cfg(test)]
#[allow(dead_code)]
fn draw_peace_symbol(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    color: egui::Color32,
    scale: f32,
) {
    let stroke = egui::Stroke::new(1.1 * scale, color);
    // Outer circle
    painter.circle_stroke(center, radius, stroke);
    // Vertical line top to bottom
    painter.line_segment(
        [
            egui::pos2(center.x, center.y - radius),
            egui::pos2(center.x, center.y + radius),
        ],
        stroke,
    );
    // Left arm: center to lower-left at ~240 degrees
    let arm_angle_l: f32 = 240.0_f32.to_radians();
    painter.line_segment(
        [
            center,
            egui::pos2(
                center.x + radius * arm_angle_l.cos(),
                center.y - radius * arm_angle_l.sin(),
            ),
        ],
        stroke,
    );
    // Right arm: center to lower-right at ~300 degrees
    let arm_angle_r: f32 = 300.0_f32.to_radians();
    painter.line_segment(
        [
            center,
            egui::pos2(
                center.x + radius * arm_angle_r.cos(),
                center.y - radius * arm_angle_r.sin(),
            ),
        ],
        stroke,
    );
}

pub fn create(
    params: Arc<GenXDelayParams>,
    editor_state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        editor_state,
        (),
        |egui_ctx, _| {
            setup_custom_fonts(egui_ctx);
        },
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

                // ── Header ──
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("GENX DELAY")
                            .heading()
                            .color(TEXT_CREAM),
                    );
                });

                ui.add_space(8.0 * scale);
                draw_adaptive_control_grid(ui, setter, &params, scale, modulation_enabled);
            });
        },
    )
}
