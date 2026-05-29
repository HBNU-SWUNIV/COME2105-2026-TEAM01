// =================================================================
mod graph;
mod algorithm;
mod performance;
mod storage;
mod visualization;
mod ui;

use eframe::egui;
use ui::AlgoSketchApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Algo-Sketch")
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Algo-Sketch",
        native_options,
        Box::new(|_cc| Box::new(AlgoSketchApp::default())),
    )
}
