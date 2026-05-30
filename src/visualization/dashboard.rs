//! egui 기반 성능 분석 대시보드 (팀원 C: 권경빈 담당)

use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};
use crate::performance::ComparisonReport;
use crate::algorithm::AlgorithmKind;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ScaleView {
    #[default] Scale100,
    Scale1000,
    Scale10000,
}

#[derive(Debug, Default)]
pub struct DashboardPanel {
    pub report:           Option<ComparisonReport>,
    pub show_predictions: bool,
    pub selected_scale:   ScaleView,
}

impl DashboardPanel {
    pub fn new() -> Self { Self::default() }

    pub fn update_report(&mut self, report: ComparisonReport) {
        self.report = Some(report);
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        // TODO: 보고서가 없으면 안내 문구 표시
        // show_predictions에 따라 바 차트 또는 라인 차트 + 통계 테이블 표시
        todo!("DashboardPanel::render 구현 예정 — 권경빈")
    }

    fn draw_bar_chart(&self, ui: &mut egui::Ui) {
        // TODO: 알고리즘별 고유 색상 바 차트 (실행 시간 비교)
        // 색상 매핑:
        //   BFS=파랑(70,130,220)  DFS=초록(80,200,120)  Dijkstra=빨강(220,100,80)
        //   BellmanFord=노랑(230,170,40)  FloydWarshall=보라(160,80,220)
        //   AStar=청록(40,210,210)  Prim=주황(255,140,0)  Kruskal=분홍(200,60,120)
        //   TopologicalSort=민트(60,220,200)  BidirectionalBFS=라벤더(180,100,255)
        todo!("draw_bar_chart 구현 예정 — 권경빈")
    }

    fn draw_line_chart(&self, ui: &mut egui::Ui) {
        // TODO: 스케일 예측 라인 차트 (노드 수 증가에 따른 예측 시간)
        // selected_scale에 따라 x축 범위 제한 (100 / 1000 / 10000)
        todo!("draw_line_chart 구현 예정 — 권경빈")
    }

    fn draw_comparison_table(&self, ui: &mut egui::Ui) {
        // TODO: 알고리즘 | 실행시간(ms) | 방문노드 | 간선탐색 그리드 테이블
        // 최하단에 fastest 알고리즘 강조 표시
        todo!("draw_comparison_table 구현 예정 — 권경빈")
    }
}
