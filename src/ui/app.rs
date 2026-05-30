//! 메인 App 구조체 (팀원 A: 엄예지 담당)

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use eframe::egui;
use egui::{Color32, Pos2, Rect, Stroke, Vec2};

use crate::{
    graph::{Graph, NodeVisualState,
            history::{GraphHistory, GraphCommand, apply_command, build_remove_node_command}},
    algorithm::{AlgorithmKind, AlgorithmResult, run_algorithm},
    performance::{PerformanceTracker, AlgorithmStats, ComparisonReport,
                  PredictionEngine, AlgorithmComplexity},
    storage::{save_graph, load_graph,
              session_history::{SessionHistory, SessionRecord}},
    visualization::DashboardPanel,
};

// ── 테마 ──────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Theme { #[default] Dark, Light }

pub struct ThemeColors {
    pub canvas_bg:              Color32,
    pub grid_line:              Color32,
    pub edge_default:           Color32,
    pub weight_label:           Color32,
    pub node_unvisited:         Color32,
    pub node_unvisited_stroke:  Color32,
    pub node_current:           Color32,
    pub node_current_stroke:    Color32,
    pub node_visited:           Color32,
    pub node_visited_stroke:    Color32,
    pub node_label:             Color32,
    pub start_marker:           Color32,
    pub drag_edge:              Color32,
    pub pseudocode_active_bg:   Color32,
    pub pseudocode_active_fg:   Color32,
    pub pseudocode_inactive_fg: Color32,
    pub status_text:            Color32,
    pub negative_cycle_warn:    Color32,
}

impl Theme {
    pub fn colors(self) -> ThemeColors {
        // TODO: Dark / Light 각각의 색상 팔레트 반환
        todo!("Theme::colors 구현 예정 — 엄예지")
    }
    pub fn apply_visuals(self, ctx: &egui::Context) {
        match self {
            Theme::Dark  => ctx.set_visuals(egui::Visuals::dark()),
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
        }
    }
}

// ── 수도코드 ──────────────────────────────────────────────────────────────────
fn pseudocode_lines(kind: AlgorithmKind) -> Vec<&'static str> {
    // TODO: 각 AlgorithmKind별 수도코드 라인 배열 반환
    // BFS, DFS, Dijkstra, BellmanFord, FloydWarshall, AStar,
    // Prim, Kruskal, TopologicalSort, BidirectionalBFS
    todo!("pseudocode_lines 구현 예정 — 엄예지")
}

// ── 앱 상태 ───────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Default)]
pub enum AppState { #[default] Editing, Running, Paused, Finished }

#[derive(Debug, Clone, PartialEq, Default)]
pub enum MouseMode { #[default] AddNode, AddEdge, Delete, Move }

pub struct AlgoSketchApp {
    // 그래프 데이터
    pub graph: Graph,

    // 알고리즘 실행 상태
    pub selected_algorithm: AlgorithmKind,
    pub algorithm_result:   Option<AlgorithmResult>,
    pub snapshot_index:     usize,
    pub app_state:          AppState,
    pub animation_speed:    f32,
    pub start_node_id:      usize,

    // 성능 분석
    pub tracker:            PerformanceTracker,
    pub comparison_report:  ComparisonReport,
    pub dashboard:          DashboardPanel,

    // UI 내부 상태
    pub mouse_mode:         MouseMode,
    pub edge_start_node:    Option<usize>,
    pub dragging_node:      Option<usize>,
    pub weight_input:       String,
    pub log_messages:       VecDeque<(String, Option<Color32>)>,
    pub node_label_counter: usize,
    pub show_weights:       bool,
    pub status_message:     String,
    pub pending_edge:       Option<(usize, usize)>,

    // 챌린지 모드
    pub challenge_mode:     bool,
    pub challenge_score:    u32,
    pub challenge_wrong:    u32,
    pub challenge_expected: Option<usize>,

    // 테마
    pub theme: Theme,

    // 추가 기능
    pub history:              GraphHistory,
    pub session_history:      SessionHistory,
    pub show_session_history: bool,
    pub last_advance_time:    Option<Instant>,
    pub mouse_on_canvas:      bool,
}

impl Default for AlgoSketchApp {
    fn default() -> Self {
        Self {
            graph:                Graph::new(),
            selected_algorithm:   AlgorithmKind::BFS,
            algorithm_result:     None,
            snapshot_index:       0,
            app_state:            AppState::Editing,
            animation_speed:      0.6,
            start_node_id:        0,
            tracker:              PerformanceTracker::new(),
            comparison_report:    ComparisonReport::new(),
            dashboard:            DashboardPanel::new(),
            mouse_mode:           MouseMode::AddNode,
            edge_start_node:      None,
            dragging_node:        None,
            weight_input:         "1".to_string(),
            log_messages:         VecDeque::new(),
            node_label_counter:   0,
            show_weights:         true,
            status_message:       "노드를 클릭하여 추가하세요".to_string(),
            pending_edge:         None,
            theme:                Theme::Dark,
            challenge_mode:       false,
            challenge_score:      0,
            challenge_wrong:      0,
            challenge_expected:   None,
            history:              GraphHistory::new(50),
            session_history:      SessionHistory::new(),
            show_session_history: false,
            last_advance_time:    None,
            mouse_on_canvas:      false,
        }
    }
}

impl AlgoSketchApp {
    fn push_log(&mut self, msg: String) {
        self.log_messages.push_back((msg, None));
        if self.log_messages.len() > 50 { self.log_messages.pop_front(); }
    }

    fn push_log_colored(&mut self, msg: String, color: Color32) {
        self.log_messages.push_back((msg, Some(color)));
        if self.log_messages.len() > 50 { self.log_messages.pop_front(); }
    }

    fn update_challenge_expected(&mut self) {
        self.challenge_expected = self.algorithm_result.as_ref()
            .and_then(|r| r.snapshots.get(self.snapshot_index))
            .map(|s| s.current_node);
    }

    fn handle_challenge_click(&mut self, clicked_node: usize) -> bool {
        // TODO: 정답 판정, 점수 갱신, 로그 출력, 다음 스냅샷 진행
        todo!("handle_challenge_click 구현 예정 — 엄예지")
    }

    fn complexity_for(&self, kind: AlgorithmKind) -> AlgorithmComplexity {
        match kind {
            AlgorithmKind::BFS | AlgorithmKind::DFS => AlgorithmComplexity::BfsOrDfs,
            AlgorithmKind::Dijkstra | AlgorithmKind::AStar => AlgorithmComplexity::Dijkstra,
            AlgorithmKind::BellmanFord => AlgorithmComplexity::BellmanFord,
            AlgorithmKind::FloydWarshall => AlgorithmComplexity::FloydWarshall,
            AlgorithmKind::Prim => AlgorithmComplexity::Prim,
            AlgorithmKind::Kruskal => AlgorithmComplexity::Kruskal,
            AlgorithmKind::TopologicalSort | AlgorithmKind::BidirectionalBFS
                => AlgorithmComplexity::BfsOrDfs,
        }
    }

    fn on_run_clicked(&mut self) {
        // TODO: 노드 없으면 경고, start_node 설정, run_algorithm 호출,
        //       PerformanceTracker 측정, SessionRecord 저장,
        //       comparison_report 갱신, app_state 전환
        todo!("on_run_clicked 구현 예정 — 엄예지")
    }

    fn advance_animation(&mut self) {
        // TODO: snapshot_index 기준으로 노드 visual_state 갱신,
        //       log 출력, 완료 시 AppState::Finished 전환
        todo!("advance_animation 구현 예정 — 엄예지")
    }

    fn reset(&mut self) {
        self.graph.reset_visual_states();
        self.algorithm_result   = None;
        self.snapshot_index     = 0;
        self.app_state          = AppState::Editing;
        self.edge_start_node    = None;
        self.status_message     = "초기화 완료".to_string();
        self.challenge_expected = None;
        self.challenge_score    = 0;
        self.challenge_wrong    = 0;
        self.last_advance_time  = None;
    }

    fn on_save_clicked(&mut self) {
        // TODO: rfd로 저장 경로 선택 후 save_graph 호출
        todo!("on_save_clicked 구현 예정 — 엄예지")
    }

    fn on_load_clicked(&mut self) {
        // TODO: rfd로 파일 선택 후 load_graph 호출, 그래프 교체
        todo!("on_load_clicked 구현 예정 — 엄예지")
    }

    // ── 패널 렌더링 ─────────────────────────────────────────────────────────
    fn render_left_panel(&mut self, ui: &mut egui::Ui) {
        // TODO: 테마 전환 버튼, 알고리즘 선택 (10종), 시간복잡도 힌트,
        //       시작 노드, 속도 슬라이더, 실행/일시정지/초기화 버튼,
        //       한 스텝 버튼, Undo/Redo 버튼, 편집 모드 선택,
        //       가중치 입력, 방향그래프 체크, 챌린지 모드, 저장/불러오기,
        //       그래프 정보, 전체 삭제 버튼
        todo!("render_left_panel 구현 예정 — 엄예지")
    }

    fn render_right_panel(&mut self, ui: &mut egui::Ui) {
        // TODO: 수도코드 패널 (현재 라인 하이라이트),
        //       음수 사이클 경고 배너,
        //       큐/스택 상태, 방문 순서, 거리 배열 표시
        todo!("render_right_panel 구현 예정 — 엄예지")
    }

    fn render_canvas(&mut self, ui: &mut egui::Ui) {
        // TODO: 배경·격자 렌더링, 간선 렌더링(화살표·가중치),
        //       드래그 간선 선, 노드 렌더링(색상·라벨·시작표시·A*목표표시),
        //       편집 모드 입력 처리, 챌린지 모드 클릭 처리
        todo!("render_canvas 구현 예정 — 엄예지")
    }

    fn handle_canvas_input(&mut self, response: &egui::Response, _rect: Rect) {
        // TODO: Move/AddNode/AddEdge/Delete 모드별 마우스 이벤트 처리
        //       노드 추가/삭제/간선 추가/삭제 시 GraphHistory에 record
        todo!("handle_canvas_input 구현 예정 — 엄예지")
    }

    fn node_at(&self, pos: Pos2) -> Option<usize> {
        // TODO: 반경 20px 이내 노드 ID 반환
        todo!("node_at 구현 예정 — 엄예지")
    }
}

fn node_label(counter: usize) -> String {
    let mut n = counter;
    let mut s = String::new();
    loop {
        s.insert(0, (b'A' + (n % 26) as u8) as char);
        if n < 26 { break; }
        n = n / 26 - 1;
    }
    s
}

fn point_to_segment_dist(p: Pos2, a: Pos2, b: Pos2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let len2 = ab.x * ab.x + ab.y * ab.y;
    if len2 == 0.0 { return ap.length(); }
    let t = ((ap.x * ab.x + ap.y * ab.y) / len2).clamp(0.0, 1.0);
    (p - (a + ab * t)).length()
}

fn rfd_lite_save() -> Option<std::path::PathBuf> {
    Some(std::path::PathBuf::from("graph.json"))
}
fn rfd_lite_open() -> Option<std::path::PathBuf> {
    let path = std::path::PathBuf::from("graph.json");
    if path.exists() { Some(path) } else { None }
}

impl eframe::App for AlgoSketchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TODO: 테마 적용, Ctrl+Z/Y 단축키 처리,
        //       타이머 기반 애니메이션 자동 진행,
        //       왼쪽/오른쪽/하단/중앙 패널 레이아웃 구성
        //       하단 패널: 성능 대시보드 탭 / 실행 히스토리 탭 전환
        todo!("AlgoSketchApp::update 구현 예정 — 엄예지")
    }
}
