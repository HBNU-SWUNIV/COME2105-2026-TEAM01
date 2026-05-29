// =================================================================
mod graph;
mod ui;

fn main() {
    let mut options = eframe::NativeOptions::default();
    
    // 맥북 에어 그래픽 충돌(wgpu panic)을 방지하기 위한 소프트웨어 백엔드 강제 지정
    options.renderer = eframe::Renderer::Glow; 
    
    options.viewport = egui::ViewportBuilder::default()
        .with_inner_size([1000.0, 700.0]);

    if let Err(e) = eframe::run_native(
        "Algo-Sketch Simulator",
        options,
        Box::new(|cc| Box::new(ui::app::TemplateApp::new(cc))),
    ) {
        eprintln!("Error: {:?}", e);
    }
}