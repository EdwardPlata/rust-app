// This file is the entry point of the application. It initializes the EGUI context and runs the main event loop for the dashboard designer.

mod app;
mod components;
mod utils;
mod widgets;

use app::DashboardApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "EGUI Dashboard Designer",
        options,
        Box::new(|_cc| Ok(Box::new(DashboardApp::default()))),
    )
}
