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
            // ── 한글 폰트 설정 ──────────────────────────────────────
            let mut fonts = egui::FontDefinitions::default();

            // Windows 시스템 폰트 경로 목록 (우선순위 순)
            let candidates = [
                "C:/Windows/Fonts/malgun.ttf",      // 맑은 고딕 (Win 7+)
                "C:/Windows/Fonts/malgunbd.ttf",    // 맑은 고딕 Bold
                "C:/Windows/Fonts/gulim.ttc",       // 굴림
                "C:/Windows/Fonts/batang.ttc",      // 바탕
                "C:/Windows/Fonts/NanumGothic.ttf", // 나눔고딕 (설치된 경우)
            ];

            let mut loaded = false;
            for path in &candidates {
                if let Ok(data) = std::fs::read(path) {
                    fonts.font_data.insert(
                        "korean".to_owned(),
                        egui::FontData::from_owned(data),
                    );
                    // Proportional 폰트 패밀리 맨 앞에 삽입 → 한글 우선 렌더링
                    fonts
                        .families
                        .entry(egui::FontFamily::Proportional)
                        .or_default()
                        .insert(0, "korean".to_owned());
                    // Monospace에도 추가
                    fonts
                        .families
                        .entry(egui::FontFamily::Monospace)
                        .or_default()
                        .push("korean".to_owned());
                    loaded = true;
                    break;
                }
            }

            if !loaded {
                eprintln!("[경고] 한글 폰트를 찾지 못했습니다. 텍스트가 깨질 수 있습니다.");
            }

            cc.egui_ctx.set_fonts(fonts);
            // ────────────────────────────────────────────────────────

            Box::new(AlgoSketchApp::default())
        }),
    )
}

