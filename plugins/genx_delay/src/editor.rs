//! Minimal GUI editor for GenX Delay

use nih_plug::prelude::*;
use nih_plug_egui::egui;
use nih_plug_egui::{create_egui_editor, EguiState};
use std::sync::Arc;

use crate::GenXDelayParams;

const WINDOW_WIDTH: u32 = 300;
const WINDOW_HEIGHT: u32 = 200;

pub fn default_state() -> Arc<EguiState> {
    EguiState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT)
}

pub fn create(
    _params: Arc<GenXDelayParams>,
    editor_state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        editor_state,
        (),
        |_, _| {},
        move |egui_ctx, _setter, _state| {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.label("GenX Delay - Test GUI");
                ui.label("If you see this, the GUI is working!");
            });
        },
    )
}
