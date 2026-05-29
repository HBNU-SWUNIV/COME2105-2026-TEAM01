use crate::graph::core::{Node, Edge, AlgorithmTracker};

pub struct TemplateApp {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub tracker: AlgorithmTracker,
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 맥/윈도우 공통 시스템 폰트를 안전하게 불러옵니다.
        let mut fonts = egui::FontDefinitions::default();
        let font_path = if cfg!(target_os = "macos") {
            "/System/Library/Fonts/Supplemental/AppleGothic.ttf"
        } else {
            "C:\\Windows\\Fonts\\malgun.ttf"
        };

        if let Ok(font_data) = std::fs::read(font_path) {
            fonts.font_data.insert(
                "hangul".to_string(),
                egui::FontData::from_owned(font_data),
            );
            if let Some(vec) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                vec.insert(0, "hangul".to_string());
            }
            if let Some(vec) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
                vec.insert(0, "hangul".to_string());
            }
            cc.egui_ctx.set_fonts(fonts);
        }

        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            tracker: AlgorithmTracker::default(),
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("💻 Algo-Sketch 시뮬레이터 공통 뼈대 빌드 성공!");
            ui.separator();
            ui.label("이 화면의 한글이 깨지지 않고 잘 보인다면 성공입니다.");
            ui.label("이제 각 팀원들은 본인 브랜치로 이동하여 코드를 채워주세요.");
            
            ui.horizontal(|ui| {
                if ui.button("알고리즘 시작 (팀원 B 테스트용)").clicked() { /* 연동 예정 */ }
                if ui.button("JSON 저장 (팀원 C 테스트용)").clicked() { /* 연동 예정 */ }
            });
        });
    }
}