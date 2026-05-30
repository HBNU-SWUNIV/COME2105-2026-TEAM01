

//! 메인 App 구조체 (팀원 A: 엄예지 담당)

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use eframe::egui;
use egui::{Color32, Pos2, Rect, Stroke, Vec2};

use crate::{
    graph::{Graph, NodeVisualState, history::{GraphHistory, GraphCommand, apply_command, build_remove_node_command}},
    algorithm::{AlgorithmKind, AlgorithmResult, run_algorithm},
    performance::{
        PerformanceTracker, AlgorithmStats, ComparisonReport,
        PredictionEngine, AlgorithmComplexity,
    },
    storage::{save_graph, load_graph, session_history::{SessionHistory, SessionRecord}},
    visualization::DashboardPanel,
};

// ── 테마 ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

/// 테마별 색상 팔레트
pub struct ThemeColors {
    pub canvas_bg:       Color32,
    pub grid_line:       Color32,
    pub edge_default:    Color32,
    pub weight_label:    Color32,
    pub node_unvisited:  Color32,
    pub node_unvisited_stroke: Color32,
    pub node_current:    Color32,
    pub node_current_stroke:   Color32,
    pub node_visited:    Color32,
    pub node_visited_stroke:   Color32,
    pub node_label:      Color32,
    pub start_marker:    Color32,
    pub drag_edge:       Color32,
    pub pseudocode_active_bg:   Color32,
    pub pseudocode_active_fg:   Color32,
    pub pseudocode_inactive_fg: Color32,
    pub status_text:     Color32,
    pub negative_cycle_warn: Color32,
}

impl Theme {
    pub fn colors(self) -> ThemeColors {
        match self {
            Theme::Dark => ThemeColors {
                canvas_bg:              Color32::from_rgb(30, 30, 40),
                grid_line:              Color32::from_rgba_premultiplied(60, 60, 80, 100),
                edge_default:           Color32::from_rgb(120, 140, 200),
                weight_label:           Color32::from_rgb(200, 220, 255),
                node_unvisited:         Color32::from_rgb(60, 90, 160),
                node_unvisited_stroke:  Color32::from_rgb(120, 160, 255),
                node_current:           Color32::from_rgb(220, 180, 0),
                node_current_stroke:    Color32::from_rgb(255, 230, 100),
                node_visited:           Color32::from_rgb(50, 160, 90),
                node_visited_stroke:    Color32::from_rgb(100, 220, 140),
                node_label:             Color32::WHITE,
                start_marker:           Color32::from_rgb(255, 120, 50),
                drag_edge:              Color32::from_rgb(255, 200, 0),
                pseudocode_active_bg:   Color32::from_rgb(255, 230, 100),
                pseudocode_active_fg:   Color32::BLACK,
                pseudocode_inactive_fg: Color32::LIGHT_GRAY,
                status_text:            Color32::from_rgb(180, 220, 255),
                negative_cycle_warn:    Color32::from_rgb(255, 80, 80),
            },
            Theme::Light => ThemeColors {
                canvas_bg:              Color32::from_rgb(240, 242, 248),
                grid_line:              Color32::from_rgba_premultiplied(180, 190, 210, 120),
                edge_default:           Color32::from_rgb(80, 100, 160),
                weight_label:           Color32::from_rgb(60, 80, 140),
                node_unvisited:         Color32::from_rgb(130, 165, 230),
                node_unvisited_stroke:  Color32::from_rgb(60, 100, 200),
                node_current:           Color32::from_rgb(240, 190, 0),
                node_current_stroke:    Color32::from_rgb(180, 130, 0),
                node_visited:           Color32::from_rgb(80, 190, 120),
                node_visited_stroke:    Color32::from_rgb(30, 130, 70),
                node_label:             Color32::from_rgb(20, 20, 40),
                start_marker:           Color32::from_rgb(220, 80, 30),
                drag_edge:              Color32::from_rgb(200, 140, 0),
                pseudocode_active_bg:   Color32::from_rgb(255, 235, 100),
                pseudocode_active_fg:   Color32::BLACK,
                pseudocode_inactive_fg: Color32::from_rgb(80, 80, 100),
                status_text:            Color32::from_rgb(40, 80, 160),
                negative_cycle_warn:    Color32::from_rgb(200, 40, 40),
            },
        }
    }

    /// egui 기본 visuals 적용
    pub fn apply_visuals(self, ctx: &egui::Context) {
        match self {
            Theme::Dark  => ctx.set_visuals(egui::Visuals::dark()),
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
        }
    }
}

// ── 수도코드 ──────────────────────────────────────────────────────────────────

fn pseudocode_lines(kind: AlgorithmKind) -> Vec<&'static str> {
    match kind {
        AlgorithmKind::BFS => vec![
            "BFS(G, start):",
            "  visited[start] = true; enqueue(start)",
            "  while queue not empty:",
            "    node = dequeue()",
            "    for each neighbor of node:",
            "      if not visited[neighbor]:",
            "        visited[neighbor] = true",
            "        enqueue(neighbor)",
        ],
        AlgorithmKind::DFS => vec![
            "DFS(G, start):",
            "  push(start) to stack",
            "  while stack not empty:",
            "    node = pop()",
            "    if not visited[node]:",
            "      visited[node] = true",
            "      for each neighbor of node:",
            "        push(neighbor)",
        ],
        AlgorithmKind::Dijkstra => vec![
            "Dijkstra(G, start):",
            "  dist[start]=0; dist[v]=∞ for others",
            "  priority_queue.push((0, start))",
            "  while pq not empty:",
            "    (d, node) = pq.pop_min()",
            "    if visited[node]: continue",
            "    visited[node] = true",
            "    for each (neighbor, w) of node:",
            "      if dist[node]+w < dist[neighbor]:",
            "        dist[neighbor] = dist[node]+w",
            "        pq.push((dist[neighbor], neighbor))",
        ],
        AlgorithmKind::BellmanFord => vec![
            "BellmanFord(G, start):",
            "  dist[start]=0; dist[v]=∞ for others",
            "  repeat (V-1) times:",
            "    for each edge (u, v, w):",
            "      if dist[u]+w < dist[v]:",
            "        dist[v] = dist[u]+w   ← 완화",
            "    if no update: break early",
            "  for each edge: check negative cycle",
        ],
        AlgorithmKind::FloydWarshall => vec![
            "FloydWarshall(G):",
            "  dist[i][j] = w(i,j) or ∞",
            "  for k in 0..V:           ← 중간 노드",
            "    for i in 0..V:",
            "      for j in 0..V:",
            "        if dist[i][k]+dist[k][j] < dist[i][j]:",
            "          dist[i][j] = dist[i][k]+dist[k][j]",
            "  if dist[i][i] < 0: negative cycle",
        ],
        AlgorithmKind::AStar => vec![
            "A*(G, start, goal):",
            "  g[start]=0; f[start]=h(start,goal)",
            "  open_set.push((f[start], start))",
            "  while open_set not empty:",
            "    node = open_set.pop_min(f)",
            "    if node == goal: return g[goal]",
            "    for each (neighbor, w) of node:",
            "      tentative_g = g[node] + w",
            "      if tentative_g < g[neighbor]:",
            "        g[neighbor] = tentative_g",
            "        f[neighbor] = g[neighbor] + h(neighbor,goal)",
            "        open_set.push((f[neighbor], neighbor))",
        ],
        AlgorithmKind::Prim => vec![
            "Prim(G, start):",
            "  key[start]=0; key[v]=∞ for others",
            "  pq.push((0, start))",
            "  while pq not empty:",
            "    node = pq.pop_min(key)",
            "    if in_mst[node]: continue",
            "    in_mst[node] = true",
            "    for each (neighbor, w) of node:",
            "      if w < key[neighbor]:",
            "        key[neighbor] = w",
            "        pq.push((w, neighbor))",
        ],
        AlgorithmKind::Kruskal => vec![
            "Kruskal(G):",
            "  sort edges by weight ascending",
            "  init UnionFind for each node",
            "  for each edge (u, v, w):",
            "    if find(u) != find(v):   ← 사이클 없음",
            "      mst.add(u, v, w)",
            "      union(u, v)",
            "    else: skip  ← 사이클 형성",
            "  (MST complete when V-1 edges added)",
        ],
        AlgorithmKind::TopologicalSort => vec![
            "TopologicalSort(G) — Kahn's Algorithm:",
            "  in_degree[] 계산",
            "  진입차수=0 노드 → queue 삽입",
            "  while queue not empty:",
            "    node = queue.pop()",
            "    result.append(node)",
            "    for neighbor in adj[node]:",
            "      in_degree[neighbor] -= 1",
            "      if in_degree[neighbor] == 0: queue.push(neighbor)",
            "  if len(result) < V: 사이클 존재 (DAG 아님)",
        ],
        AlgorithmKind::BidirectionalBFS => vec![
            "BidirectionalBFS(G, start, goal):",
            "  fwd_queue=[start]; bwd_queue=[goal]",
            "  fwd_visited[start]=true; bwd_visited[goal]=true",
            "  while both queues not empty:",
            "    node = fwd_queue.pop()",
            "    for neighbor in adj[node]:",
            "      if not fwd_visited: enqueue fwd",
            "      if in bwd_visited: 교차 노드 → 종료",
            "    node = bwd_queue.pop() (역방향 반복)",
            "  경로 = forward_path + backward_path",
        ],
    }
}

// ── 앱 상태 ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AppState {
    #[default]
    Editing,
    Running,
    Paused,
    Finished,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum MouseMode {
    #[default]
    AddNode,
    AddEdge,
    Delete,
    Move,
}

pub struct AlgoSketchApp {
    // 그래프 데이터
    pub graph: Graph,

    // 알고리즘 실행 상태
    pub selected_algorithm: AlgorithmKind,
    pub algorithm_result: Option<AlgorithmResult>,
    pub snapshot_index: usize,
    pub app_state: AppState,
    pub animation_speed: f32,
    pub start_node_id: usize,

    // 성능 분석
    pub tracker: PerformanceTracker,
    pub comparison_report: ComparisonReport,
    pub dashboard: DashboardPanel,

    // UI 내부 상태
    pub mouse_mode: MouseMode,
    pub edge_start_node: Option<usize>,
    pub dragging_node: Option<usize>,
    pub weight_input: String,
    pub log_messages: VecDeque<(String, Option<Color32>)>,
    pub node_label_counter: usize,
    pub show_weights: bool,
    pub status_message: String,
    pub pending_edge: Option<(usize, usize)>,

    // 챌린지 모드
    pub challenge_mode: bool,
    pub challenge_score: u32,        // 연속 정답 수
    pub challenge_wrong: u32,        // 누적 오답 수
    pub challenge_expected: Option<usize>, // 이번 스텝에서 기대하는 노드 ID

    // 테마
    pub theme: Theme,

    // ── 추가 기능 ─────────────────────────────────────────────────────
    pub history: GraphHistory,
    pub session_history: SessionHistory,
    pub show_session_history: bool,
    pub last_advance_time: Option<Instant>,
    pub mouse_on_canvas: bool,
}

impl Default for AlgoSketchApp {
    fn default() -> Self {
        Self {
            graph: Graph::new(),
            selected_algorithm: AlgorithmKind::BFS,
            algorithm_result: None,
            snapshot_index: 0,
            app_state: AppState::Editing,
            animation_speed: 0.6,
            start_node_id: 0,
            tracker: PerformanceTracker::new(),
            comparison_report: ComparisonReport::new(),
            dashboard: DashboardPanel::new(),
            mouse_mode: MouseMode::AddNode,
            edge_start_node: None,
            dragging_node: None,
            weight_input: "1".to_string(),
            log_messages: VecDeque::new(),
            node_label_counter: 0,
            show_weights: true,
            status_message: "노드를 클릭하여 추가하세요".to_string(),
            pending_edge: None,
            theme: Theme::Dark,
            challenge_mode: false,
            challenge_score: 0,
            challenge_wrong: 0,
            challenge_expected: None,
            history: GraphHistory::new(50),
            session_history: SessionHistory::new(),
            show_session_history: false,
            last_advance_time: None,
            mouse_on_canvas: false,
        }
    }
}

impl AlgoSketchApp {
    fn push_log(&mut self, msg: String) {
        self.log_messages.push_back((msg, None));
        if self.log_messages.len() > 50 {
            self.log_messages.pop_front();
        }
    }

    fn push_log_colored(&mut self, msg: String, color: Color32) {
        self.log_messages.push_back((msg, Some(color)));
        if self.log_messages.len() > 50 {
            self.log_messages.pop_front();
        }
    }

    /// 현재 snapshot_index 기준으로 이번에 사용자가 클릭해야 할 노드 ID를 계산합니다.
    /// 스냅샷의 current_node를 정답으로 사용합니다.
    fn update_challenge_expected(&mut self) {
        self.challenge_expected = self.algorithm_result
            .as_ref()
            .and_then(|r| r.snapshots.get(self.snapshot_index))
            .map(|s| s.current_node);
    }

    /// 챌린지 모드에서 캔버스 클릭을 처리합니다.
    /// 반환값: true = 정답, false = 오답 또는 예상 없음
    fn handle_challenge_click(&mut self, clicked_node: usize) -> bool {
        let expected = match self.challenge_expected {
            Some(e) => e,
            None => return false,
        };

        if clicked_node == expected {
            // 정답
            self.challenge_score += 1;
            let label = self.graph.nodes.get(clicked_node)
                .map(|n| n.label.clone())
                .unwrap_or_else(|| clicked_node.to_string());
            self.push_log_colored(
                format!("✅ 정답입니다! 노드 {} | 연속 정답: {}", label, self.challenge_score),
                Color32::from_rgb(80, 200, 120),
            );
            // 스냅샷 한 칸 전진
            self.advance_animation();
            // 완료 여부 확인
            if self.app_state == AppState::Finished {
                self.push_log_colored(
                    format!("🏆 챌린지 완료! 정답 {} 회, 오답 {} 회", self.challenge_score, self.challenge_wrong),
                    Color32::from_rgb(80, 200, 120),
                );
                self.status_message = format!(
                    "🏆 챌린지 완료! 정답 {} · 오답 {}",
                    self.challenge_score, self.challenge_wrong
                );
                self.challenge_expected = None;
            } else {
                // 다음 정답 노드 갱신
                self.update_challenge_expected();
                let next_label = self.challenge_expected
                    .and_then(|id| self.graph.nodes.get(id))
                    .map(|n| n.label.clone())
                    .unwrap_or_default();
                self.status_message = format!(
                    "🎯 정답! ({}) | 다음 노드를 클릭하세요  [정답:{} / 오답:{}]",
                    label, self.challenge_score, self.challenge_wrong
                );
                // 챌린지 모드에서는 항상 Paused 유지
                if self.app_state == AppState::Running {
                    self.app_state = AppState::Paused;
                }
            }
            true
        } else {
            // 오답
            self.challenge_wrong += 1;
            let clicked_label = self.graph.nodes.get(clicked_node)
                .map(|n| n.label.clone())
                .unwrap_or_else(|| clicked_node.to_string());
            let expected_hint = self.algorithm_result
                .as_ref()
                .and_then(|r| r.snapshots.get(self.snapshot_index.saturating_sub(1)))
                .map(|s| {
                    let q: Vec<String> = s.queue_or_stack.iter()
                        .filter_map(|&id| self.graph.nodes.get(id).map(|n| n.label.clone()))
                        .collect();
                    q.join(", ")
                })
                .unwrap_or_default();
            self.push_log_colored(
                format!("❌ 틀렸습니다. (선택: {}) 수도코드와 현재 큐/스택 상태를 다시 확인하세요.", clicked_label),
                Color32::from_rgb(220, 80, 80),
            );
            self.status_message = format!(
                "❌ 틀렸습니다! 큐/스택: [{}]  [정답:{} / 오답:{}]",
                expected_hint, self.challenge_score, self.challenge_wrong
            );
            false
        }
    }

    fn node_at(&self, pos: Pos2) -> Option<usize> {
        self.graph.nodes.iter().position(|n| {
            let dx = n.x - pos.x;
            let dy = n.y - pos.y;
            (dx * dx + dy * dy).sqrt() < 20.0
        })
    }

    fn complexity_for(&self, kind: AlgorithmKind) -> AlgorithmComplexity {
        match kind {
            AlgorithmKind::BFS | AlgorithmKind::DFS => AlgorithmComplexity::BfsOrDfs,
            AlgorithmKind::Dijkstra | AlgorithmKind::AStar => AlgorithmComplexity::Dijkstra,
            AlgorithmKind::BellmanFord => AlgorithmComplexity::BellmanFord,
            AlgorithmKind::FloydWarshall => AlgorithmComplexity::FloydWarshall,
            AlgorithmKind::Prim => AlgorithmComplexity::Prim,
            AlgorithmKind::Kruskal => AlgorithmComplexity::Kruskal,
            AlgorithmKind::TopologicalSort | AlgorithmKind::BidirectionalBFS => AlgorithmComplexity::BfsOrDfs,
        }
    }

    fn on_run_clicked(&mut self) {
        if self.graph.nodes.is_empty() {
            self.status_message = "노드가 없습니다.".to_string();
            return;
        }
        let start = self.start_node_id.min(self.graph.nodes.len() - 1);
        self.graph.reset_visual_states();
        self.tracker.start();
        let result = run_algorithm(&self.graph, start, self.selected_algorithm);
        let perf = self.tracker.stop(&result);

        let complexity = self.complexity_for(self.selected_algorithm);
        let engine = PredictionEngine::new(
            perf.elapsed_ms,
            self.graph.nodes.len(),
            self.graph.edges.len(),
        );
        let predictions = engine.generate_scale_predictions(complexity);

        self.comparison_report.add(AlgorithmStats {
            kind: self.selected_algorithm,
            performance: perf.clone(),
            predictions,
        });
        self.dashboard.update_report(self.comparison_report.clone());

        // 세션 히스토리 기록
        let visit_order_clone = result.visit_order.clone();
        let edge_cnt = result.edge_traversal_count;
        let snap_cnt = result.snapshots.len();
        let record = SessionRecord::new(
            0,
            self.selected_algorithm,
            start,
            &self.graph,
            perf.elapsed_ms,
            visit_order_clone,
            edge_cnt,
            snap_cnt,
        );
        let _ = self.session_history.append(record);

        self.push_log(format!("▶ {} 실행 시작 (노드{}에서)", self.selected_algorithm, start));
        self.algorithm_result = Some(result);
        self.snapshot_index = 0;
        self.last_advance_time = None; // 타이머 리셋
        if self.challenge_mode {
            self.app_state = AppState::Paused;
            self.challenge_score = 0;
            self.challenge_wrong = 0;
            self.update_challenge_expected();
            let node_label = self.challenge_expected
                .and_then(|id| self.graph.nodes.get(id))
                .map(|n| n.label.clone())
                .unwrap_or_default();
            self.status_message = format!(
                "🎯 챌린지 모드 | 다음 방문할 노드를 캔버스에서 클릭하세요!"
            );
            self.push_log(format!("🎯 챌린지 모드 시작 — 첫 번째 정점을 클릭하세요"));
        } else {
            self.app_state = AppState::Running;
            self.status_message = format!("{} 실행 중...", self.selected_algorithm);
        }
    }

    fn advance_animation(&mut self) {
        let total = self.algorithm_result.as_ref().map(|r| r.snapshots.len()).unwrap_or(0);
        if total == 0 { self.app_state = AppState::Finished; return; }

        if self.snapshot_index < total {
            let snap = self.algorithm_result.as_ref().unwrap().snapshots[self.snapshot_index].clone();

            let n = self.graph.nodes.len();
            for i in 0..n {
                if i < snap.visited_flags.len() {
                    self.graph.nodes[i].visual_state = if snap.visited_flags[i] {
                        if i == snap.current_node { NodeVisualState::Current }
                        else { NodeVisualState::Visited }
                    } else {
                        NodeVisualState::Unvisited
                    };
                }
            }

            // 음수 사이클 경고
            if snap.negative_cycle {
                self.status_message = "⚠ 음수 사이클 감지됨!".to_string();
            }

            self.push_log(snap.log_message.clone());
            self.snapshot_index += 1;
        }

        if self.snapshot_index >= total {
            for node in &mut self.graph.nodes {
                node.visual_state = NodeVisualState::Visited;
            }
            self.app_state = AppState::Finished;
            if !self.status_message.contains("음수 사이클") {
                self.status_message = format!("{} 완료!", self.selected_algorithm);
            }
            self.push_log(format!("✅ {} 완료 (총 {} 스텝)", self.selected_algorithm, total));
        }
    }

    fn reset(&mut self) {
        self.graph.reset_visual_states();
        self.algorithm_result = None;
        self.snapshot_index = 0;
        self.app_state = AppState::Editing;
        self.edge_start_node = None;
        self.status_message = "초기화 완료".to_string();
        self.challenge_expected = None;
        self.challenge_score = 0;
        self.challenge_wrong = 0;
    }

    fn on_save_clicked(&mut self) {
        if let Some(path) = rfd_lite_save() {
            match save_graph(&self.graph, &path, "my_graph") {
                Ok(_)  => self.status_message = format!("저장 완료: {}", path.display()),
                Err(e) => self.status_message = format!("저장 실패: {}", e),
            }
        }
    }

    fn on_load_clicked(&mut self) {
        if let Some(path) = rfd_lite_open() {
            match load_graph(&path) {
                Ok((graph, _data)) => {
                    self.graph = graph;
                    self.reset();
                    self.status_message = "불러오기 완료".to_string();
                }
                Err(e) => self.status_message = format!("불러오기 실패: {}", e),
            }
        }
    }

    // ── 패널 렌더링 ──────────────────────────────────────────────────────────

    fn render_left_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙ Algo-Caster");
        ui.separator();

        // ── 테마 전환 버튼 ──
        ui.horizontal(|ui| {
            let (icon, label) = match self.theme {
                Theme::Dark  => ("☀", "라이트 모드"),
                Theme::Light => ("🌙", "다크 모드"),
            };
            if ui.button(format!("{} {}", icon, label)).clicked() {
                self.theme = match self.theme {
                    Theme::Dark  => Theme::Light,
                    Theme::Light => Theme::Dark,
                };
            }
        });

        ui.separator();

        // ── 알고리즘 선택 ──
        ui.label("알고리즘 선택");
        ui.label(egui::RichText::new("▸ 탐색").weak());
        ui.selectable_value(&mut self.selected_algorithm, AlgorithmKind::BFS,      "BFS (너비 우선)");
        ui.selectable_value(&mut self.selected_algorithm, AlgorithmKind::DFS,      "DFS (깊이 우선)");
        ui.label(egui::RichText::new("▸ 최단 경로").weak());
        ui.selectable_value(&mut self.selected_algorithm, AlgorithmKind::Dijkstra,     "Dijkstra");
        ui.selectable_value(&mut self.selected_algorithm, AlgorithmKind::BellmanFord,  "Bellman-Ford ±w");
        ui.selectable_value(&mut self.selected_algorithm, AlgorithmKind::FloydWarshall,"Floyd-Warshall");
        ui.selectable_value(&mut self.selected_algorithm, AlgorithmKind::AStar,        "A* (휴리스틱)");
        ui.label(egui::RichText::new("▸ 최소 신장 트리 (MST)").weak());
        ui.selectable_value(&mut self.selected_algorithm, AlgorithmKind::Prim,    "Prim (프림)");
        ui.selectable_value(&mut self.selected_algorithm, AlgorithmKind::Kruskal, "Kruskal (크루스칼)");
        ui.label(egui::RichText::new("▸ 고급 알고리즘").weak());
        ui.selectable_value(&mut self.selected_algorithm, AlgorithmKind::TopologicalSort,  "위상 정렬 (Topo Sort)");
        ui.selectable_value(&mut self.selected_algorithm, AlgorithmKind::BidirectionalBFS, "양방향 BFS");

        // 알고리즘별 짧은 설명
        let hint = match self.selected_algorithm {
            AlgorithmKind::BFS           => "O(V+E) · 비가중치 최단경로",
            AlgorithmKind::DFS           => "O(V+E) · 경로/사이클 탐색",
            AlgorithmKind::Dijkstra      => "O(E log V) · 음수 가중치 ✗",
            AlgorithmKind::BellmanFord   => "O(V·E) · 음수 가중치 ✓",
            AlgorithmKind::FloydWarshall => "O(V³) · 전체 쌍 최단경로",
            AlgorithmKind::AStar         => "O(E log V) · 목표 지향 탐색",
            AlgorithmKind::Prim          => "O(E log V) · MST · 연결 그래프",
            AlgorithmKind::Kruskal       => "O(E log E) · MST · Union-Find",
            AlgorithmKind::TopologicalSort  => "O(V+E) · DAG 위상 정렬",
            AlgorithmKind::BidirectionalBFS => "O(b^(d/2)) · 양방향 최단경로",
        };
        ui.label(egui::RichText::new(hint).small().weak().italics());

        ui.separator();

        // 시작 노드
        ui.horizontal(|ui| {
            ui.label("시작 노드:");
            ui.add(egui::DragValue::new(&mut self.start_node_id)
                .clamp_range(0..=self.graph.nodes.len().saturating_sub(1)));
        });

        // A* 목표 노드 안내
        if self.selected_algorithm == AlgorithmKind::AStar {
            ui.label(egui::RichText::new(
                "A* 목표: 마지막 노드 (자동)"
            ).small().weak());
        }

        // 애니메이션 속도
        ui.horizontal(|ui| {
            ui.label("속도(초/스텝):");
            ui.add(egui::Slider::new(&mut self.animation_speed, 0.05..=2.0).logarithmic(true));
        });

        ui.separator();

        // 실행 버튼들
        ui.horizontal(|ui| {
            let run_label = if self.app_state == AppState::Running { "⏸ 일시정지" }
                            else if self.app_state == AppState::Paused { "▶ 재개" }
                            else { "▶ 실행" };

            if ui.button(run_label).clicked() {
                match self.app_state {
                    AppState::Editing | AppState::Finished => self.on_run_clicked(),
                    AppState::Running  => self.app_state = AppState::Paused,
                    AppState::Paused   => self.app_state = AppState::Running,
                }
            }

            if ui.button("↺ 초기화").clicked() {
                self.reset();
            }
        });

        if self.app_state == AppState::Paused {
            if ui.button("→ 한 스텝").clicked() {
                self.advance_animation();
                if self.app_state != AppState::Finished {
                    self.app_state = AppState::Paused;
                }
            }
        }

        ui.separator();

        // ── Undo / Redo ──
        ui.label(egui::RichText::new("↩ Undo / Redo").strong());
        ui.horizontal(|ui| {
            let can_undo = self.history.can_undo();
            let can_redo = self.history.can_redo();
            if ui.add_enabled(can_undo, egui::Button::new("↩ Undo ")).clicked() {
                if let Some(desc) = self.history.undo(&mut self.graph) {
                    self.push_log(format!("↩ Undo: {}", desc));
                    self.status_message = format!("Undo: {}", desc);
                }
            }
            if ui.add_enabled(can_redo, egui::Button::new("↪ Redo ")).clicked() {
                if let Some(desc) = self.history.redo(&mut self.graph) {
                    self.push_log(format!("↪ Redo: {}", desc));
                    self.status_message = format!("Redo: {}", desc);
                }
            }
        });
        if self.history.can_undo() {
            if let Some(last) = self.history.undo_history().first() {
                ui.label(egui::RichText::new(format!("마지막 작업: {}", last)).small().weak());
            }
        }

        ui.separator();

        // 마우스 모드
        ui.label("✏ 편집 모드");
        ui.selectable_value(&mut self.mouse_mode, MouseMode::AddNode, "🔵 노드 추가");
        ui.selectable_value(&mut self.mouse_mode, MouseMode::AddEdge, "➡ 간선 추가");
        ui.selectable_value(&mut self.mouse_mode, MouseMode::Delete,  "🗑 삭제");
        ui.selectable_value(&mut self.mouse_mode, MouseMode::Move,    "✥ 이동");

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("기본 가중치:");
            ui.text_edit_singleline(&mut self.weight_input);
        });
        ui.checkbox(&mut self.graph.is_directed, "방향 그래프");
        ui.checkbox(&mut self.show_weights, "가중치 표시");

        ui.separator();

        // ── 챌린지 모드 ──
        ui.label(egui::RichText::new("🎯 챌린지 모드").strong());
        ui.checkbox(&mut self.challenge_mode, "챌린지 모드 활성화");
        if self.challenge_mode {
            ui.label(egui::RichText::new(
                "실행 후 다음 방문할 노드를
캔버스에서 직접 클릭하세요."
            ).small().weak());
            if self.app_state != AppState::Editing {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("✅ {}", self.challenge_score))
                        .color(egui::Color32::from_rgb(80, 200, 120)).strong());
                    ui.label(egui::RichText::new(format!("❌ {}", self.challenge_wrong))
                        .color(egui::Color32::from_rgb(220, 80, 80)).strong());
                });
            }
        }

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("💾 저장").clicked() { self.on_save_clicked(); }
            if ui.button("📂 불러오기").clicked() { self.on_load_clicked(); }
        });

        ui.separator();

        ui.label(egui::RichText::new("📊 그래프 정보").strong());
        ui.label(format!("노드 수: {}", self.graph.nodes.len()));
        ui.label(format!("간선 수: {}", self.graph.edges.len()));
        if let Some(r) = &self.algorithm_result {
            ui.label(format!("총 스텝: {}", r.snapshots.len()));
            ui.label(format!("현재 스텝: {}/{}", self.snapshot_index, r.snapshots.len()));
        }

        ui.separator();

        if ui.button("🗑 그래프 전체 삭제").clicked() {
            self.graph = Graph::new();
            self.reset();
            self.node_label_counter = 0;
        }
    }

    fn render_right_panel(&mut self, ui: &mut egui::Ui) {
        let colors = self.theme.colors();

        // ── 동적 폰트 크기: 패널 너비에 비례 (기준 230px → 기본 13pt, 최소 10, 최대 22)
        let panel_width = ui.available_width();
        let code_font_size = (panel_width / 230.0 * 13.0).clamp(10.0, 22.0);
        let info_font_size = (panel_width / 230.0 * 12.0).clamp(9.0, 18.0);

        ui.heading("📋 수도코드");
        ui.separator();

        let lines = pseudocode_lines(self.selected_algorithm);
        let current_line = self.algorithm_result.as_ref()
            .and_then(|r| r.snapshots.get(self.snapshot_index.saturating_sub(1)))
            .map(|s| s.current_line)
            .unwrap_or(usize::MAX);

        for (i, line) in lines.iter().enumerate() {
            let is_active = i == current_line;
            let text = egui::RichText::new(*line)
                .monospace()
                .size(code_font_size);
            if is_active {
                let text = text
                    .background_color(colors.pseudocode_active_bg)
                    .color(colors.pseudocode_active_fg)
                    .strong();
                ui.label(text);
            } else {
                ui.label(text.color(colors.pseudocode_inactive_fg));
            }
        }

        ui.separator();

        // 음수 사이클 경고 배너
        if let Some(snap) = self.algorithm_result.as_ref()
            .and_then(|r| r.snapshots.get(self.snapshot_index.saturating_sub(1)))
        {
            if snap.negative_cycle {
                ui.colored_label(
                    colors.negative_cycle_warn,
                    "⚠ 음수 사이클이 감지되어 최단 경로를 보장할 수 없습니다!",
                );
                ui.separator();
            }
        }

        // 큐/스택 상태
        if let Some(snap) = self.algorithm_result.as_ref()
            .and_then(|r| r.snapshots.get(self.snapshot_index.saturating_sub(1)))
        {
            let label = match self.selected_algorithm {
                AlgorithmKind::BFS            => "큐",
                AlgorithmKind::DFS            => "스택",
                AlgorithmKind::Dijkstra       => "우선순위 큐",
                AlgorithmKind::BellmanFord    => "처리 중 간선",
                AlgorithmKind::FloydWarshall  => "중간 노드 k",
                AlgorithmKind::AStar          => "오픈셋",
                AlgorithmKind::Prim           => "우선순위 큐 (key)",
                AlgorithmKind::Kruskal        => "처리 중 간선 (from→to)",
                AlgorithmKind::TopologicalSort  => "진입차수 0 큐",
                AlgorithmKind::BidirectionalBFS => "정방향/역방향 큐",
            };
            ui.label(egui::RichText::new(format!("{}:", label)).strong().size(info_font_size));
            let content: Vec<String> = snap.queue_or_stack.iter()
                .map(|&id| self.graph.nodes.get(id).map(|n| n.label.clone()).unwrap_or(id.to_string()))
                .collect();
            ui.label(egui::RichText::new(format!("[{}]", content.join(", "))).size(info_font_size));

            ui.label(egui::RichText::new("방문 순서:").strong().size(info_font_size));
            let visited: Vec<String> = snap.visited_order.iter()
                .map(|&id| self.graph.nodes.get(id).map(|n| n.label.clone()).unwrap_or(id.to_string()))
                .collect();
            ui.label(egui::RichText::new(visited.join(" → ")).size(info_font_size));

            // 거리 배열 (Dijkstra / Bellman-Ford / Floyd-Warshall / A*)
            if let Some(dists) = &snap.distances {
                ui.separator();
                let dist_label = match self.selected_algorithm {
                    AlgorithmKind::AStar => "g(실제 비용):",
                    AlgorithmKind::FloydWarshall => "거리 (시작 노드 기준):",
                    _ => "거리 배열:",
                };
                ui.label(egui::RichText::new(dist_label).strong());
                egui::Grid::new("dist_grid").striped(true).show(ui, |ui| {
                    ui.label("노드"); ui.label("거리"); ui.end_row();
                    for (i, d) in dists.iter().enumerate() {
                        if let Some(node) = self.graph.nodes.get(i) {
                            ui.label(&node.label);
                            if d.is_infinite() {
                                ui.label("∞");
                            } else {
                                ui.label(format!("{:.1}", d));
                            }
                            ui.end_row();
                        }
                    }
                });
            }
        }
    }

    fn render_canvas(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(
            ui.available_size(),
            egui::Sense::click_and_drag(),
        );

        let rect = response.rect;
        let colors = self.theme.colors();

        // 배경
        painter.rect_filled(rect, 0.0, colors.canvas_bg);

        // 격자
        if self.app_state == AppState::Editing || self.app_state == AppState::Paused {
            let grid_size = 40.0;
            let mut x = rect.left();
            while x < rect.right() {
                painter.line_segment(
                    [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                    Stroke::new(0.5, colors.grid_line),
                );
                x += grid_size;
            }
            let mut y = rect.top();
            while y < rect.bottom() {
                painter.line_segment(
                    [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                    Stroke::new(0.5, colors.grid_line),
                );
                y += grid_size;
            }
        }

        // 간선
        for edge in &self.graph.edges {
            let from = match self.graph.nodes.get(edge.from) { Some(n) => n, None => continue };
            let to   = match self.graph.nodes.get(edge.to)   { Some(n) => n, None => continue };
            let fp = Pos2::new(from.x, from.y);
            let tp = Pos2::new(to.x,   to.y);

            painter.line_segment([fp, tp], Stroke::new(2.0, colors.edge_default));

            if self.graph.is_directed {
                let dir = (tp - fp).normalized();
                let arrow_tip = tp - dir * 22.0;
                let perp = Vec2::new(-dir.y, dir.x) * 7.0;
                painter.line_segment([arrow_tip, arrow_tip - dir * 12.0 + perp],
                    Stroke::new(2.0, colors.edge_default));
                painter.line_segment([arrow_tip, arrow_tip - dir * 12.0 - perp],
                    Stroke::new(2.0, colors.edge_default));
            }

            if self.show_weights {
                let mid = Pos2::new((fp.x + tp.x) / 2.0, (fp.y + tp.y) / 2.0);
                painter.text(mid, egui::Align2::CENTER_CENTER,
                    format!("{:.0}", edge.weight),
                    egui::FontId::proportional(12.0),
                    colors.weight_label);
            }
        }

        // 간선 드래그 선
        if self.mouse_mode == MouseMode::AddEdge {
            if let Some(start_id) = self.edge_start_node {
                if let Some(from_node) = self.graph.nodes.get(start_id) {
                    if let Some(ptr) = response.hover_pos() {
                        painter.line_segment(
                            [Pos2::new(from_node.x, from_node.y), ptr],
                            Stroke::new(2.0, colors.drag_edge),
                        );
                    }
                }
            }
        }

        // 노드
        let node_radius = 20.0;
        for node in &self.graph.nodes {
            let center = Pos2::new(node.x, node.y);

            let (fill, stroke_color) = match node.visual_state {
                NodeVisualState::Unvisited => (colors.node_unvisited, colors.node_unvisited_stroke),
                NodeVisualState::Current   => (colors.node_current,   colors.node_current_stroke),
                NodeVisualState::Visited   => (colors.node_visited,   colors.node_visited_stroke),
            };

            if node.id == self.start_node_id.min(self.graph.nodes.len().saturating_sub(1)) {
                painter.circle_stroke(center, node_radius + 4.0, Stroke::new(2.5, colors.start_marker));
            }

            if self.edge_start_node == Some(node.id) {
                painter.circle_stroke(center, node_radius + 4.0, Stroke::new(2.5, colors.drag_edge));
            }

            // A* 목표 노드 표시
            if self.selected_algorithm == AlgorithmKind::AStar
                && node.id == self.graph.nodes.len().saturating_sub(1)
                && node.id != self.start_node_id
            {
                painter.circle_stroke(center, node_radius + 6.0,
                    Stroke::new(2.5, Color32::from_rgb(180, 80, 220)));
            }

            painter.circle_filled(center, node_radius, fill);
            painter.circle_stroke(center, node_radius, Stroke::new(2.0, stroke_color));
            painter.text(center, egui::Align2::CENTER_CENTER,
                &node.label,
                egui::FontId::proportional(14.0),
                colors.node_label);
        }

        if self.app_state == AppState::Editing {
            self.handle_canvas_input(&response, rect);
        }

        // 챌린지 모드: Paused 상태에서 노드 클릭 처리
        if self.challenge_mode
            && self.app_state == AppState::Paused
            && self.challenge_expected.is_some()
        {
            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    if let Some(hit) = self.node_at(pos) {
                        self.handle_challenge_click(hit);
                    }
                }
            }
        }
    }

    fn handle_canvas_input(&mut self, response: &egui::Response, _rect: Rect) {
        if self.mouse_mode == MouseMode::Move {
            if response.drag_started() {
                if let Some(pos) = response.interact_pointer_pos() {
                    self.dragging_node = self.node_at(pos);
                }
            }
            if response.dragged() {
                if let Some(id) = self.dragging_node {
                    let delta = response.drag_delta();
                    if let Some(node) = self.graph.nodes.get_mut(id) {
                        node.x += delta.x;
                        node.y += delta.y;
                    }
                }
            }
            if response.drag_stopped() {
                self.dragging_node = None;
            }
            return;
        }

        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                match self.mouse_mode {
                    MouseMode::AddNode => {
                        if self.node_at(pos).is_none() {
                            let label = node_label(self.node_label_counter);
                            self.node_label_counter += 1;
                            let id = self.graph.nodes.len();
                            let cmd = GraphCommand::AddNode { id, x: pos.x, y: pos.y, label: label.clone() };
                            apply_command(&mut self.graph, &cmd);
                            self.history.record(cmd);
                            self.push_log(format!("노드 {} 추가", label));
                            self.status_message = format!("노드 {} 추가됨", label);
                        }
                    }
                    MouseMode::AddEdge => {
                        if let Some(hit) = self.node_at(pos) {
                            if let Some(from) = self.edge_start_node {
                                if from != hit {
                                    let w: f64 = self.weight_input.parse().unwrap_or(1.0);
                                    let cmd = GraphCommand::AddEdge { from, to: hit, weight: w };
                                    apply_command(&mut self.graph, &cmd);
                                    self.history.record(cmd);
                                    let from_label = self.graph.nodes[from].label.clone();
                                    let to_label   = self.graph.nodes[hit].label.clone();
                                    self.push_log(format!("간선 {}→{} (가중치:{}) 추가", from_label, to_label, w));
                                    self.status_message = format!("간선 {}→{} 추가됨", from_label, to_label);
                                }
                                self.edge_start_node = None;
                            } else {
                                self.edge_start_node = Some(hit);
                                self.status_message = format!(
                                    "출발 노드 {} 선택됨. 도착 노드 클릭",
                                    self.graph.nodes[hit].label
                                );
                            }
                        } else {
                            self.edge_start_node = None;
                        }
                    }
                    MouseMode::Delete => {
                        if let Some(hit) = self.node_at(pos) {
                            if let Some(cmd) = build_remove_node_command(&self.graph, hit) {
                                let label = self.graph.nodes[hit].label.clone();
                                apply_command(&mut self.graph, &cmd);
                                self.history.record(cmd);
                                // ID 재정렬
                                for e in &mut self.graph.edges {
                                    if e.from > hit { e.from -= 1; }
                                    if e.to   > hit { e.to   -= 1; }
                                }
                                for (i, n) in self.graph.nodes.iter_mut().enumerate() { n.id = i; }
                                self.push_log(format!("노드 {} 삭제", label));
                                self.status_message = format!("노드 {} 삭제됨", label);
                                self.start_node_id = self.start_node_id.min(
                                    self.graph.nodes.len().saturating_sub(1)
                                );
                            }
                        }
                    }
                    MouseMode::Move => {}
                }
            }
        }

        if response.secondary_clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let mut closest_edge: Option<usize> = None;
                let mut closest_dist = 12.0f32;
                for (i, edge) in self.graph.edges.iter().enumerate() {
                    if let (Some(from), Some(to)) = (self.graph.nodes.get(edge.from), self.graph.nodes.get(edge.to)) {
                        let fp = Pos2::new(from.x, from.y);
                        let tp = Pos2::new(to.x, to.y);
                        let d = point_to_segment_dist(pos, fp, tp);
                        if d < closest_dist {
                            closest_dist = d;
                            closest_edge = Some(i);
                        }
                    }
                }
                if let Some(idx) = closest_edge {
                    let e = &self.graph.edges[idx];
                    let cmd = GraphCommand::RemoveEdge {
                        from: e.from, to: e.to, weight: e.weight, edge_index: idx,
                    };
                    self.push_log(format!("간선 {}→{} 삭제", e.from, e.to));
                    apply_command(&mut self.graph, &cmd);
                    self.history.record(cmd);
                    self.status_message = "간선 삭제됨".to_string();
                }
            }
        }
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
    let closest = a + ab * t;
    (p - closest).length()
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
        // 테마 적용 (매 프레임 — 전환 즉시 반영)
        self.theme.apply_visuals(ctx);

        // ── 키보드 단축키 처리 ────────────────────────────────────────
        let ctrl = ctx.input(|i| i.modifiers.ctrl);
        let z_pressed = ctx.input(|i| i.key_pressed(egui::Key::Z));
        let y_pressed = ctx.input(|i| i.key_pressed(egui::Key::Y));

        if ctrl && z_pressed && self.app_state == AppState::Editing {
            if let Some(desc) = self.history.undo(&mut self.graph) {
                self.push_log(format!("↩ Undo: {}", desc));
                self.status_message = format!("Undo: {}", desc);
            }
        }
        if ctrl && y_pressed && self.app_state == AppState::Editing {
            if let Some(desc) = self.history.redo(&mut self.graph) {
                self.push_log(format!("↪ Redo: {}", desc));
                self.status_message = format!("Redo: {}", desc);
            }
        }

        // 애니메이션 자동 진행 (타이머 기반 — 마우스 이동과 무관하게 고정 속도 유지)
        if self.app_state == AppState::Running {
            ctx.request_repaint_after(Duration::from_millis(16)); // 60fps 폴링
            let should_advance = match self.last_advance_time {
                None => true,
                Some(t) => t.elapsed() >= Duration::from_secs_f32(self.animation_speed),
            };
            if should_advance {
                self.last_advance_time = Some(Instant::now());
                self.advance_animation();
            }
        }

        // ── 왼쪽 패널 ────────────────────────────────────────────────────────
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(210.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_left_panel(ui);
                });
            });

        // ── 오른쪽 패널 ───────────────────────────────────────────────────────
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .min_width(180.0)
            .max_width(600.0)
            .default_width(230.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_right_panel(ui);
                });
            });

        // ── 하단 패널 ────────────────────────────────────────────────────────
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .min_height(120.0)
            .max_height(600.0)
            .default_height(200.0)
            .show(ctx, |ui| {
                // ── 동적 폰트 크기: 패널 높이에 비례 (기준 200px → 기본 11pt, 최소 9, 최대 18)
                let panel_height = ui.available_height();
                let log_font_size = (panel_height / 200.0 * 11.0).clamp(9.0, 18.0);
                let stat_font_size = (panel_height / 200.0 * 12.0).clamp(9.0, 18.0);
                // 로그 스크롤 영역 높이도 패널 크기에 맞게 동적 조절
                let log_scroll_height = (panel_height - 60.0).max(60.0);

                let status_color = self.theme.colors().status_text;
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(&self.status_message)
                            .color(status_color)
                            .strong()
                            .size(stat_font_size)
                    );
                });
                ui.separator();

                // ── 하단 탭 ──────────────────────────────────────────
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.show_session_history, false, "📊 성능 대시보드");
                    ui.selectable_value(&mut self.show_session_history, true,  "📋 실행 히스토리");
                });
                ui.separator();

                ui.columns(2, |cols| {
                    cols[0].label(egui::RichText::new("📜 실행 로그").strong().size(stat_font_size));
                    egui::ScrollArea::vertical()
                        .id_source("log_scroll")
                        .max_height(log_scroll_height)
                        .stick_to_bottom(true)
                        .show(&mut cols[0], |ui| {
                            for (msg, color) in &self.log_messages {
                                let text = egui::RichText::new(msg.as_str()).monospace().size(log_font_size);
                                let text = match color {
                                    Some(c) => text.color(*c),
                                    None    => text,
                                };
                                ui.label(text);
                            }
                        });

                    if !self.show_session_history {
                        // 성능 대시보드
                        let report_clone = self.comparison_report.clone();
                        self.dashboard.update_report(report_clone);
                        self.dashboard.render(&mut cols[1]);
                    } else {
                        // 실행 히스토리 패널
                        cols[1].label(egui::RichText::new("📋 알고리즘 실행 히스토리").strong().size(stat_font_size));
                        let records: Vec<_> = self.session_history.recent_records().into_iter().take(20).collect();
                        if records.is_empty() {
                            cols[1].label(egui::RichText::new("아직 실행 기록이 없습니다. 알고리즘을 실행하면 여기에 표시됩니다.").weak());
                        } else {
                            // 통계 요약
                            let stats = self.session_history.stats_summary();
                            cols[1].horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("총 {}회 실행", stats.record_count)).small());
                                ui.separator();
                                ui.label(egui::RichText::new(format!("평균 {:.2}ms", stats.avg_elapsed_ms)).small());
                                if let Some(k) = stats.most_used {
                                    ui.separator();
                                    ui.label(egui::RichText::new(format!("최다: {}", k)).small());
                                }
                            });
                            egui::ScrollArea::vertical()
                                .id_source("hist_scroll")
                                .max_height(log_scroll_height)
                                .show(&mut cols[1], |ui| {
                                    egui::Grid::new("hist_grid")
                                        .striped(true)
                                        .min_col_width(50.0)
                                        .show(ui, |ui| {
                                            ui.label(egui::RichText::new("알고리즘").strong().size(log_font_size));
                                            ui.label(egui::RichText::new("시작").strong().size(log_font_size));
                                            ui.label(egui::RichText::new("노드").strong().size(log_font_size));
                                            ui.label(egui::RichText::new("간선").strong().size(log_font_size));
                                            ui.label(egui::RichText::new("시간(ms)").strong().size(log_font_size));
                                            ui.label(egui::RichText::new("방문").strong().size(log_font_size));
                                            ui.label(egui::RichText::new("스텝").strong().size(log_font_size));
                                            ui.end_row();
                                            for r in &records {
                                                ui.label(egui::RichText::new(format!("{}", r.algorithm)).size(log_font_size));
                                                ui.label(egui::RichText::new(format!("{}", r.start_node)).size(log_font_size));
                                                ui.label(egui::RichText::new(format!("{}", r.node_count)).size(log_font_size));
                                                ui.label(egui::RichText::new(format!("{}", r.edge_count)).size(log_font_size));
                                                ui.label(egui::RichText::new(format!("{:.2}", r.elapsed_ms)).size(log_font_size));
                                                ui.label(egui::RichText::new(format!("{}", r.visit_order.len())).size(log_font_size));
                                                ui.label(egui::RichText::new(format!("{}", r.snapshot_count)).size(log_font_size));
                                                ui.end_row();
                                            }
                                        });
                                });
                        }
                    }
                });
            });

        // ── 중앙 캔버스 ───────────────────────────────────────────────────────
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                self.render_canvas(ui);
            });
    }
}
