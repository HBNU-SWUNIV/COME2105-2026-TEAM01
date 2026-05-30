//! 진입점 및 한글 폰트 설정 (팀원 A: 엄예지 담당)

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
            .with_inner_size([1280.0, 760.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Algo-Sketch",
        native_options,
        Box::new(|cc| {
            // TODO: Windows 한글 폰트 로드 및 egui에 등록
            // candidates: malgun.ttf, gulim.ttc, batang.ttc 등
            Box::new(AlgoSketchApp::default())
        }),
    )
}
