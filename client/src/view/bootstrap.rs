use crate::controller::Controller;
use super::ChatApp;

use eframe::egui;

/// Bootstraps the View and starts the Ui
pub struct View;


impl View {
    /*
    pub fn new(controller: Controller) -> Self {
        Self { controller }
    }
    */

    /// Runs the Ui
    pub fn run(&self) {
        
        // loads the GUI
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(800.0, 600.0)),
            ..Default::default()
        };
        eframe::run_native(
            "Beta 2",
            options,
            Box::new(|_cc| Box::new(ChatApp::new())),
        );
    }
}
