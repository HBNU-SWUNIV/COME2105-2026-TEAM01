//! egui 기반 성능 분석 대시보드 (팀원 C: 권경빈 담당)

use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};
use crate::performance::ComparisonReport;
use crate::algorithm::AlgorithmKind;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ScaleView {
    #[default]
    Scale100,
    Scale1000,
    Scale10000,
}

#[derive(Debug, Default)]
pub struct DashboardPanel {
    pub report: Option<ComparisonReport>,
    pub show_predictions: bool,
    pub selected_scale: ScaleView,
}

impl DashboardPanel {
    pub fn new() -> Self { Self::default() }

    pub fn update_report(&mut self, report: ComparisonReport) {
        self.report = Some(report);
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        if self.report.is_none() {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new("알고리즘을 실행하면 성능 분석 결과가 여기에 표시됩니다.")
                    .color(egui::Color32::GRAY));
            });
            return;
        }

        ui.horizontal(|ui| {
            // 예측 토글
            ui.checkbox(&mut self.show_predictions, "스케일 예측 표시");
            ui.separator();
            ui.label("예측 시나리오:");
            ui.selectable_value(&mut self.selected_scale, ScaleView::Scale100, "100노드");
            ui.selectable_value(&mut self.selected_scale, ScaleView::Scale1000, "1000노드");
            ui.selectable_value(&mut self.selected_scale, ScaleView::Scale10000, "10000노드");
        });

        ui.separator();

        ui.columns(2, |cols| {
            // 왼쪽: 실행 시간 Bar Chart
            cols[0].label(egui::RichText::new("⏱ 실행 시간 비교 (ms)").strong());
            self.draw_bar_chart(&mut cols[0]);

            // 오른쪽: 예측 Line Chart 또는 통계 테이블
            if self.show_predictions {
                cols[1].label(egui::RichText::new("📈 노드 수 증가에 따른 예측 실행 시간").strong());
                self.draw_line_chart(&mut cols[1]);
            } else {
                cols[1].label(egui::RichText::new("📊 알고리즘 비교 통계").strong());
                self.draw_comparison_table(&mut cols[1]);
            }
        });
    }

    fn draw_bar_chart(&self, ui: &mut egui::Ui) {
        let report = match &self.report { Some(r) => r, None => return };


        let algo_colors = [
            (AlgorithmKind::BFS,              egui::Color32::from_rgb( 70, 130, 220)),  // 파랑
            (AlgorithmKind::DFS,              egui::Color32::from_rgb( 80, 200, 120)),  // 초록
            (AlgorithmKind::Dijkstra,         egui::Color32::from_rgb(220, 100,  80)),  // 빨강
            (AlgorithmKind::BellmanFord,      egui::Color32::from_rgb(230, 170,  40)),  // 노랑
            (AlgorithmKind::FloydWarshall,    egui::Color32::from_rgb(160,  80, 220)),  // 보라
            (AlgorithmKind::AStar,            egui::Color32::from_rgb( 40, 210, 210)),  // 청록
            (AlgorithmKind::Prim,             egui::Color32::from_rgb(255, 140,   0)),  // 주황
            (AlgorithmKind::Kruskal,          egui::Color32::from_rgb(200,  60, 120)),  // 분홍
            (AlgorithmKind::TopologicalSort,  egui::Color32::from_rgb( 60, 220, 200)),  // 민트
            (AlgorithmKind::BidirectionalBFS, egui::Color32::from_rgb(180, 100, 255)),  // 라벤더
        ];

        let bars: Vec<Bar> = report.entries.iter().enumerate().map(|(i, entry)| {
            let color = algo_colors.iter()
                .find(|(k, _)| *k == entry.kind)
                .map(|(_, c)| *c)
                .unwrap_or(egui::Color32::WHITE);

            Bar::new(i as f64, entry.performance.elapsed_ms.max(0.001))
                .name(format!("{}", entry.kind))
                .fill(color)
                .width(0.6)
        }).collect();

        let chart = BarChart::new(bars).name("실행 시간");

        Plot::new("bar_chart_time")
            .height(140.0)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .show_axes([false, true])
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(chart);
            });
    }

    fn draw_line_chart(&self, ui: &mut egui::Ui) {
        let report = match &self.report { Some(r) => r, None => return };

        let algo_colors = [
            (AlgorithmKind::BFS,          egui::Color32::from_rgb( 70, 130, 220)),  // 파랑
            (AlgorithmKind::DFS,          egui::Color32::from_rgb( 80, 200, 120)),  // 초록
            (AlgorithmKind::Dijkstra,     egui::Color32::from_rgb(220, 100,  80)),  // 빨강
            (AlgorithmKind::BellmanFord,  egui::Color32::from_rgb(230, 170,  40)),  // 노랑
            (AlgorithmKind::FloydWarshall,egui::Color32::from_rgb(160,  80, 220)),  // 보라
            (AlgorithmKind::AStar,        egui::Color32::from_rgb( 40, 210, 210)),  // 청록
            (AlgorithmKind::Prim,         egui::Color32::from_rgb(255, 140,   0)),  // 주황
            (AlgorithmKind::Kruskal,      egui::Color32::from_rgb(200,  60, 120)),  // 분홍
        ];

        // 선택한 시나리오에 따라 x축 최대 노드 수를 제한
        let max_nodes = match self.selected_scale {
            ScaleView::Scale100   =>   100usize,
            ScaleView::Scale1000  =>  1000usize,
            ScaleView::Scale10000 => 10000usize,
        };

        Plot::new("line_chart_predict")
            .height(140.0)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .include_x(0.0)
            .include_x(max_nodes as f64)
            .include_y(0.0)
            .show(ui, |plot_ui| {
                for entry in &report.entries {
                    if entry.predictions.is_empty() { continue; }
                    let color = algo_colors.iter()
                        .find(|(k, _)| *k == entry.kind)
                        .map(|(_, c)| *c)
                        .unwrap_or(egui::Color32::WHITE);

                    // selected_scale 이하의 포인트만 필터링
                    let points: PlotPoints = entry.predictions.iter()
                        .filter(|p| p.node_count <= max_nodes)
                        .map(|p| [p.node_count as f64, p.predicted_ms])
                        .collect();

                    if points.points().is_empty() { continue; }

                    plot_ui.line(
                        Line::new(points)
                            .name(format!("{}", entry.kind))
                            .color(color)
                            .width(2.0)
                    );
                }
            });
    }

    fn draw_comparison_table(&self, ui: &mut egui::Ui) {
        let report = match &self.report { Some(r) => r, None => return };

        egui::Grid::new("comparison_grid")
            .striped(true)
            .min_col_width(80.0)
            .show(ui, |ui| {
                // 헤더
                ui.label(egui::RichText::new("알고리즘").strong());
                ui.label(egui::RichText::new("실행 시간(ms)").strong());
                ui.label(egui::RichText::new("방문 노드").strong());
                ui.label(egui::RichText::new("간선 탐색").strong());
                ui.end_row();

                for entry in &report.entries {
                    ui.label(format!("{}", entry.kind));
                    ui.label(format!("{:.3}", entry.performance.elapsed_ms));
                    ui.label(format!("{}", entry.performance.visited_count));
                    ui.label(format!("{}", entry.performance.edge_traversal_count));
                    ui.end_row();
                }

                // 최적 결과 요약
                if let Some(fastest) = report.fastest() {
                    ui.separator();
                    ui.end_row();
                    ui.label(egui::RichText::new("🏆 최고 속도").color(egui::Color32::GOLD));
                    ui.label(format!("{}", fastest.kind));
                    ui.end_row();
                }
            });
    }
}