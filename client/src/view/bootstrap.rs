use eframe::egui;

// use crate::presenter::Presenter;

use super::ChatApp;

/// Bootstraps the View and starts the Ui
pub struct View;

impl View {
    /// Runs the Ui
    pub fn run(&self) {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(800.0, 600.0)),
            ..Default::default()
        };
        eframe::run_native(
            "My egui App",
            options,
            Box::new(|_cc| Box::new(ChatApp::new())),
        );
    }
}